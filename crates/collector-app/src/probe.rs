//! `collector-app --probe host[:unit]`: reads one device's SunSpec map once and returns
//! it as a [`DeviceDump`], for device reports and `sunspec-sim --fixture`.

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use modbus_client::ModbusClient;
use poller_actor::{base_candidates, discover_models};
use sunspec_parser::{decode_model, model_definition, PointType};
use types::{DeviceDump, DeviceIdentity, ModelDump, DEVICE_DUMP_FORMAT};

use crate::CollectorConfig;

/// Connects with the device's effective settings (including its static-device
/// overrides), walks the model chain and reads every model once. A model that fails to
/// read is kept with its error. Unless `keep_identifiers` is set, serial numbers and
/// network addresses are blanked so the dump can be shared.
pub async fn probe(
    config: &CollectorConfig,
    identity: DeviceIdentity,
    keep_identifiers: bool,
) -> Result<DeviceDump> {
    let device = config.device_config(identity.clone());
    let mut modbus = device.modbus.clone();
    modbus.host = identity.ip.clone();
    modbus.timeout_ms = device.actor.request_timeout.as_millis() as u64;
    let port = modbus.port;

    let client = ModbusClient::connect(modbus)
        .await
        .with_context(|| format!("connect to {}:{port}", identity.ip))?;
    let (base, models) = discover_models(
        &client,
        identity.unit_id,
        &base_candidates(device.base_address),
    )
    .await
    .with_context(|| format!("find the SunSpec model chain of {}", identity.key()))?;

    let mut redacted = Vec::new();
    let mut dumps = Vec::with_capacity(models.len());
    for model in models {
        let (mut registers, error) = match client
            .read_range(identity.unit_id, model.start, model.length)
            .await
        {
            Ok(registers) => (registers, None),
            Err(err) => (Vec::new(), Some(err.to_string())),
        };
        if !keep_identifiers {
            redacted.extend(redact_identifiers(model.id, &mut registers));
        }
        let points = decode_model(model.id, &registers).unwrap_or_default();
        dumps.push(ModelDump {
            id: model.id,
            name: model.name,
            start: model.start,
            length: model.length,
            registers,
            points,
            error,
        });
    }

    Ok(DeviceDump {
        format: DEVICE_DUMP_FORMAT,
        collector_version: env!("CARGO_PKG_VERSION").to_string(),
        probed_at_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        host: identity.ip,
        port,
        unit_id: identity.unit_id,
        base_address: base,
        redacted,
        models: dumps,
    })
}

/// Sets top-level serial number (`SN`) and network address points (MAC, IPv4, IPv6) to
/// their "not implemented" value, and returns the `<model>.<point>` names it changed.
/// Points inside repeating groups (for example battery module serials) are not covered.
fn redact_identifiers(model_id: u16, registers: &mut [u16]) -> Vec<String> {
    let Some(model) = model_definition(model_id) else {
        return Vec::new();
    };
    let mut changed = Vec::new();
    let mut offset = 0usize;
    for point in &model.group.points {
        let size = usize::from(point.size);
        let blank = match point.kind {
            PointType::String if point.name == "SN" => Some(0),
            PointType::Ipaddr | PointType::Ipv6addr => Some(0),
            PointType::Eui48 => Some(u16::MAX),
            _ => None,
        };
        if let (Some(blank), Some(raw)) = (blank, registers.get_mut(offset..offset + size)) {
            if raw.iter().any(|register| *register != blank) {
                raw.fill(blank);
                changed.push(format!("{model_id}.{}", point.name));
            }
        }
        offset += size;
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serial_numbers_and_macs_are_blanked() {
        let common = model_definition(1).unwrap();
        let mut registers = vec![0x4141u16; common.group.fixed_size()];
        let changed = redact_identifiers(1, &mut registers);
        assert_eq!(changed, vec!["1.SN".to_string()]);
        let (offset, sn) = common.point_offset("SN").unwrap();
        let offset = usize::from(offset);
        assert!(registers[offset..offset + usize::from(sn.size)]
            .iter()
            .all(|register| *register == 0));
        let (manufacturer, _) = common.point_offset("Mn").unwrap();
        assert_eq!(registers[usize::from(manufacturer)], 0x4141, "Mn is kept");

        let ethernet = model_definition(11).unwrap();
        let mut registers = vec![0x0102u16; ethernet.group.fixed_size()];
        assert!(redact_identifiers(11, &mut registers)
            .iter()
            .any(|name| name == "11.MAC"));
        assert!(
            redact_identifiers(11, &mut registers).is_empty(),
            "already blank"
        );
    }
}
