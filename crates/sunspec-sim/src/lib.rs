//! A small SunSpec Modbus TCP device simulator.
//!
//! It serves holding registers (function 0x03) for one or more unit IDs, each with a
//! SunSpec model chain, and supports fault injection (stalls, dropped connections,
//! going offline) so the collector can be tested without hardware.

use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use tracing::{debug, info};

/// Protocol limit for a single read holding registers (0x03) request.
pub const MAX_REGISTERS_PER_READ: u16 = 125;

const SUNSPEC_ID0: u16 = 0x5375;
const SUNSPEC_ID1: u16 = 0x6e53;
const SUNSPEC_END: u16 = 0xFFFF;

const FC_READ_HOLDING_REGISTERS: u8 = 0x03;
const EX_ILLEGAL_FUNCTION: u8 = 0x01;
const EX_ILLEGAL_DATA_ADDRESS: u8 = 0x02;
const EX_ILLEGAL_DATA_VALUE: u8 = 0x03;
const EX_GATEWAY_TARGET_FAILED: u8 = 0x0B;

/// One SunSpec model in a device's model chain: its ID and body length (excluding the
/// two header registers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelSpec {
    pub id: u16,
    pub len: u16,
}

/// How reads of addresses outside the register map are answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unmapped {
    /// Modbus exception 0x02, as most real devices do.
    Exception,
    /// Zeros, for lenient clients that over-read.
    Zeros,
}

/// A simulated Modbus unit (one SunSpec device behind the listener).
#[derive(Debug, Clone)]
pub struct UnitSpec {
    pub unit_id: u8,
    pub base: u16,
    pub models: Vec<ModelSpec>,
}

impl UnitSpec {
    /// A three-phase inverter: common model (1) plus model 103.
    pub fn inverter(unit_id: u8) -> Self {
        Self {
            unit_id,
            base: 40_000,
            models: vec![ModelSpec { id: 1, len: 66 }, ModelSpec { id: 103, len: 50 }],
        }
    }

    pub fn with_models(unit_id: u8, models: Vec<ModelSpec>) -> Self {
        Self {
            unit_id,
            base: 40_000,
            models,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimConfig {
    pub units: Vec<UnitSpec>,
    pub unmapped: Unmapped,
}

impl SimConfig {
    pub fn single(unit: UnitSpec) -> Self {
        Self {
            units: vec![unit],
            unmapped: Unmapped::Exception,
        }
    }
}

/// Parses a model chain such as `1,103,160:48`. A bare ID uses the body length of the
/// bundled SunSpec definition; `id:len` sets it explicitly (e.g. to size repeating
/// groups such as MPPT modules).
pub fn parse_models(spec: &str) -> Result<Vec<ModelSpec>, String> {
    spec.split(',')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let entry = entry.trim();
            let (id, len) = match entry.split_once(':') {
                Some((id, len)) => (id, Some(len)),
                None => (entry, None),
            };
            let id: u16 = id
                .parse()
                .map_err(|_| format!("invalid model id in '{entry}'"))?;
            let len = match len {
                Some(len) => len
                    .parse()
                    .map_err(|_| format!("invalid model length in '{entry}'"))?,
                None => default_body_len(id)
                    .ok_or_else(|| format!("model {id} has no bundled definition; use {id}:len"))?,
            };
            Ok(ModelSpec { id, len })
        })
        .collect()
}

/// Body length (excluding the 2 header registers) of a bundled model definition.
pub fn default_body_len(model_id: u16) -> Option<u16> {
    let model = sunspec_parser::model_definition(model_id)?;
    u16::try_from(model.group.fixed_size().checked_sub(2)?).ok()
}

#[derive(Debug, Default)]
struct UnitMap {
    registers: HashMap<u16, u16>,
    model_starts: HashMap<u16, u16>,
}

impl UnitMap {
    fn build(unit: &UnitSpec) -> Self {
        let mut map = UnitMap::default();
        let mut addr = unit.base;
        for value in [SUNSPEC_ID0, SUNSPEC_ID1] {
            map.registers.insert(addr, value);
            addr = addr.wrapping_add(1);
        }
        for model in &unit.models {
            map.model_starts.entry(model.id).or_insert(addr);
            map.registers.insert(addr, model.id);
            map.registers.insert(addr.wrapping_add(1), model.len);
            for offset in 0..model.len {
                map.registers.insert(addr.wrapping_add(2 + offset), 0);
            }
            addr = addr.wrapping_add(2 + model.len);
        }
        map.registers.insert(addr, SUNSPEC_END);
        map.registers.insert(addr.wrapping_add(1), 0);
        map
    }
}

struct Inner {
    addr: SocketAddr,
    unmapped: Unmapped,
    units: Mutex<HashMap<u8, UnitMap>>,
    stalled: AtomicBool,
    connections: AtomicU64,
    requests: AtomicU64,
    /// Bumped to drop every open connection.
    drop_tx: watch::Sender<u64>,
    /// `true` while the listener should accept connections.
    online_tx: watch::Sender<bool>,
}

/// Handle to a running simulator. Cloning shares the same simulator.
#[derive(Clone)]
pub struct SimHandle {
    inner: Arc<Inner>,
}

/// Starts a simulator listening on `bind` (use port 0 for an ephemeral port).
pub async fn start(config: SimConfig, bind: SocketAddr) -> io::Result<SimHandle> {
    let listener = TcpListener::bind(bind).await?;
    let addr = listener.local_addr()?;
    let units = config
        .units
        .iter()
        .map(|unit| (unit.unit_id, UnitMap::build(unit)))
        .collect();
    let (drop_tx, _) = watch::channel(0u64);
    let (online_tx, _) = watch::channel(true);
    let handle = SimHandle {
        inner: Arc::new(Inner {
            addr,
            unmapped: config.unmapped,
            units: Mutex::new(units),
            stalled: AtomicBool::new(false),
            connections: AtomicU64::new(0),
            requests: AtomicU64::new(0),
            drop_tx,
            online_tx,
        }),
    };
    let online = handle.inner.online_tx.subscribe();
    tokio::spawn(run_listener(handle.clone(), Some(listener), online));
    info!(%addr, "sunspec simulator listening");
    Ok(handle)
}

impl SimHandle {
    pub fn local_addr(&self) -> SocketAddr {
        self.inner.addr
    }

    pub fn port(&self) -> u16 {
        self.inner.addr.port()
    }

    /// Address of the model header (the register holding the model ID).
    pub fn model_start(&self, unit_id: u8, model_id: u16) -> Option<u16> {
        let units = self.inner.units.lock().expect("sim state poisoned");
        units.get(&unit_id)?.model_starts.get(&model_id).copied()
    }

    pub fn set_register(&self, unit_id: u8, addr: u16, value: u16) {
        self.set_registers(unit_id, addr, &[value]);
    }

    pub fn set_registers(&self, unit_id: u8, addr: u16, values: &[u16]) {
        let mut units = self.inner.units.lock().expect("sim state poisoned");
        let unit = units.entry(unit_id).or_default();
        for (offset, value) in values.iter().enumerate() {
            unit.registers
                .insert(addr.wrapping_add(offset as u16), *value);
        }
    }

    /// Writes a top-level point of a model by name (raw registers, big-endian words).
    /// Returns `false` if the unit, model or point does not exist.
    pub fn set_point(&self, unit_id: u8, model_id: u16, point: &str, values: &[u16]) -> bool {
        let Some(start) = self.model_start(unit_id, model_id) else {
            return false;
        };
        let Some((offset, _)) =
            sunspec_parser::model_definition(model_id).and_then(|model| model.point_offset(point))
        else {
            return false;
        };
        self.set_registers(unit_id, start.wrapping_add(offset), values);
        true
    }

    /// Writes a string point, NUL-padded to its size.
    pub fn set_text(&self, unit_id: u8, model_id: u16, point: &str, value: &str) -> bool {
        let Some((_, def)) =
            sunspec_parser::model_definition(model_id).and_then(|model| model.point_offset(point))
        else {
            return false;
        };
        let mut bytes = value.as_bytes().to_vec();
        bytes.resize(usize::from(def.size) * 2, 0);
        let words: Vec<u16> = bytes
            .chunks(2)
            .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
            .collect();
        self.set_point(unit_id, model_id, point, &words)
    }

    /// While stalled, requests are read but never answered.
    pub fn set_stalled(&self, stalled: bool) {
        self.inner.stalled.store(stalled, Ordering::SeqCst);
    }

    /// Closes every open client connection; new connections are still accepted.
    pub fn drop_connections(&self) {
        self.inner
            .drop_tx
            .send_modify(|generation| *generation += 1);
    }

    /// Stops listening (connections are refused) and drops open connections.
    pub fn go_offline(&self) {
        self.inner.online_tx.send_replace(false);
        self.drop_connections();
    }

    /// Starts listening again on the same address.
    pub fn go_online(&self) {
        let online = self.inner.online_tx.subscribe();
        if !self.inner.online_tx.send_replace(true) {
            tokio::spawn(run_listener(self.clone(), None, online));
        }
    }

    pub fn connections_accepted(&self) -> u64 {
        self.inner.connections.load(Ordering::SeqCst)
    }

    pub fn requests_served(&self) -> u64 {
        self.inner.requests.load(Ordering::SeqCst)
    }

    fn read(&self, unit_id: u8, start: u16, count: u16) -> Result<Vec<u16>, u8> {
        let units = self.inner.units.lock().expect("sim state poisoned");
        let unit = units.get(&unit_id).ok_or(EX_GATEWAY_TARGET_FAILED)?;
        (0..count)
            .map(|offset| {
                let addr = start.checked_add(offset).ok_or(EX_ILLEGAL_DATA_ADDRESS)?;
                match unit.registers.get(&addr) {
                    Some(value) => Ok(*value),
                    None if self.inner.unmapped == Unmapped::Zeros => Ok(0),
                    None => Err(EX_ILLEGAL_DATA_ADDRESS),
                }
            })
            .collect()
    }
}

/// `online` is subscribed before the task is spawned, so a `go_offline` that races the
/// task's startup is still observed.
async fn run_listener(
    handle: SimHandle,
    listener: Option<TcpListener>,
    mut online: watch::Receiver<bool>,
) {
    let listener = match listener {
        Some(listener) => listener,
        None => match TcpListener::bind(handle.inner.addr).await {
            Ok(listener) => listener,
            Err(err) => {
                info!(error = %err, "sunspec simulator failed to rebind");
                return;
            }
        },
    };
    loop {
        if !*online.borrow_and_update() {
            return;
        }
        tokio::select! {
            accepted = listener.accept() => {
                let Ok((stream, peer)) = accepted else { continue };
                handle.inner.connections.fetch_add(1, Ordering::SeqCst);
                debug!(%peer, "simulator accepted connection");
                tokio::spawn(serve_connection(handle.clone(), stream));
            }
            changed = online.changed() => {
                if changed.is_err() || !*online.borrow() {
                    return;
                }
            }
        }
    }
}

async fn serve_connection(handle: SimHandle, mut stream: TcpStream) {
    let mut drop_rx = handle.inner.drop_tx.subscribe();
    loop {
        tokio::select! {
            frame = read_frame(&mut stream) => {
                let Ok((header, pdu)) = frame else { return };
                if handle.inner.stalled.load(Ordering::SeqCst) {
                    continue;
                }
                handle.inner.requests.fetch_add(1, Ordering::SeqCst);
                let response = respond(&handle, header.unit_id, &pdu);
                if write_frame(&mut stream, header, &response).await.is_err() {
                    return;
                }
            }
            _ = drop_rx.changed() => return,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct MbapHeader {
    transaction_id: u16,
    unit_id: u8,
}

async fn read_frame(stream: &mut TcpStream) -> io::Result<(MbapHeader, Vec<u8>)> {
    let mut header = [0u8; 7];
    stream.read_exact(&mut header).await?;
    let transaction_id = u16::from_be_bytes([header[0], header[1]]);
    let length = u16::from_be_bytes([header[4], header[5]]);
    if length < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "short MBAP length",
        ));
    }
    let mut pdu = vec![0u8; usize::from(length) - 1];
    stream.read_exact(&mut pdu).await?;
    Ok((
        MbapHeader {
            transaction_id,
            unit_id: header[6],
        },
        pdu,
    ))
}

async fn write_frame(stream: &mut TcpStream, header: MbapHeader, pdu: &[u8]) -> io::Result<()> {
    let length = u16::try_from(pdu.len() + 1).expect("pdu fits in a frame");
    let mut frame = Vec::with_capacity(7 + pdu.len());
    frame.extend_from_slice(&header.transaction_id.to_be_bytes());
    frame.extend_from_slice(&0u16.to_be_bytes());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.push(header.unit_id);
    frame.extend_from_slice(pdu);
    stream.write_all(&frame).await
}

fn respond(handle: &SimHandle, unit_id: u8, pdu: &[u8]) -> Vec<u8> {
    let function = pdu.first().copied().unwrap_or_default();
    if function != FC_READ_HOLDING_REGISTERS || pdu.len() != 5 {
        return vec![function | 0x80, EX_ILLEGAL_FUNCTION];
    }
    let start = u16::from_be_bytes([pdu[1], pdu[2]]);
    let count = u16::from_be_bytes([pdu[3], pdu[4]]);
    if count == 0 || count > MAX_REGISTERS_PER_READ {
        return vec![function | 0x80, EX_ILLEGAL_DATA_VALUE];
    }
    match handle.read(unit_id, start, count) {
        Ok(values) => {
            let mut response = Vec::with_capacity(2 + values.len() * 2);
            response.push(function);
            response.push((values.len() * 2) as u8);
            for value in values {
                response.extend_from_slice(&value.to_be_bytes());
            }
            response
        }
        Err(code) => vec![function | 0x80, code],
    }
}

/// Fills plausible nameplate data and scale factors, then updates live inverter values
/// once per second until the simulator is dropped. Used by the `sunspec-sim` binary.
pub fn run_demo(handle: SimHandle, unit_ids: Vec<u8>) {
    for &unit in &unit_ids {
        handle.set_text(unit, 1, "Mn", "Rusty SunSpec");
        handle.set_text(unit, 1, "Md", "SIM-10K");
        handle.set_text(unit, 1, "SN", &format!("SIM{unit:05}"));
        handle.set_point(unit, 1, "DA", &[u16::from(unit)]);
        for (point, exponent) in [
            ("A_SF", -2i16),
            ("V_SF", -1),
            ("W_SF", 0),
            ("Hz_SF", -2),
            ("WH_SF", 0),
        ] {
            handle.set_point(unit, 103, point, &[exponent as u16]);
        }
    }
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(std::time::Duration::from_secs(1));
        let mut energy_wh = 1_000_000u32;
        let mut seconds = 0u32;
        loop {
            tick.tick().await;
            seconds = seconds.wrapping_add(1);
            for &unit in &unit_ids {
                // A slow sine-like ramp so dashboards show movement.
                let phase = (seconds % 120) as f64 / 120.0 * std::f64::consts::TAU;
                let watts = (5_000.0 + 2_000.0 * phase.sin() + f64::from(unit) * 100.0) as u16;
                let amps_centi = (f64::from(watts) / 230.0 / 3.0 * 100.0) as u16;
                handle.set_point(unit, 103, "W", &[watts]);
                handle.set_point(unit, 103, "A", &[amps_centi]);
                handle.set_point(unit, 103, "PhVphA", &[2301 + (seconds % 7) as u16]);
                handle.set_point(unit, 103, "Hz", &[5000 + (seconds % 5) as u16]);
                handle.set_point(unit, 103, "St", &[4]);
                energy_wh = energy_wh.wrapping_add(u32::from(watts) / 3_600 + 1);
                handle.set_point(
                    unit,
                    103,
                    "WH",
                    &[(energy_wh >> 16) as u16, energy_wh as u16],
                );
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_model_ids_use_the_bundled_definition_length() {
        let models = parse_models("1,103,701,713,705,160:48").unwrap();
        let lengths: Vec<(u16, u16)> = models.iter().map(|model| (model.id, model.len)).collect();
        // 705's curves repeat by NCrv/NPt, so a bare ID serves only its fixed part.
        assert_eq!(
            lengths,
            vec![
                (1, 66),
                (103, 50),
                (701, 153),
                (713, 7),
                (705, 13),
                (160, 48)
            ]
        );
        assert!(parse_models("9999").is_err(), "unknown models need id:len");
    }
}
