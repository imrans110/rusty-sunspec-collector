#![allow(dead_code)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use thiserror::Error;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{timeout, Duration};
use tracing::{debug, info, warn};

use types::DeviceIdentity;

#[cfg_attr(feature = "config", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub subnet: String,
    pub port: u16,
    pub max_concurrency: usize,
    pub per_host_timeout_ms: u64,
    /// List of Modbus Unit IDs to assume for found hosts.
    pub unit_ids: Vec<u8>,
    /// Optional static device list. When set, subnet scanning is skipped.
    pub static_devices: Vec<DeviceIdentity>,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            subnet: "192.168.1.0/24".to_string(),
            port: 502,
            max_concurrency: 64,
            per_host_timeout_ms: 200,
            unit_ids: vec![1],
            static_devices: Vec::new(),
        }
    }
}

#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("invalid subnet {0}")]
    InvalidSubnet(String),
    #[error("max_concurrency must be >= 1")]
    InvalidConcurrency,
    #[error("scan task failed: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
}

pub async fn discover(config: DiscoveryConfig) -> Result<Vec<DeviceIdentity>, DiscoveryError> {
    if !config.static_devices.is_empty() {
        info!(
            count = config.static_devices.len(),
            "using static discovery list"
        );
        return Ok(config.static_devices);
    }

    discover_subnet(config).await
}

pub async fn discover_subnet(
    config: DiscoveryConfig,
) -> Result<Vec<DeviceIdentity>, DiscoveryError> {
    if config.max_concurrency == 0 {
        return Err(DiscoveryError::InvalidConcurrency);
    }

    let (first, last) = parse_subnet_range(&config.subnet)?;
    info!(
        subnet = %config.subnet,
        port = config.port,
        "starting subnet discovery"
    );

    let semaphore = Arc::new(Semaphore::new(config.max_concurrency));
    let mut join_set = JoinSet::new();
    let mut devices = Vec::new();
    let mut current = first;
    // Capture unit_ids to move into tasks (needs to be cloned or shared)
    // Since Vec<u8> is cheap, we can clone it per task or wrap in Arc. Arc is better for many tasks.
    let unit_ids = Arc::new(config.unit_ids);

    loop {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed");
        let ip = u32_to_ipv4(current);
        let port = config.port;
        let timeout_ms = config.per_host_timeout_ms;
        let task_unit_ids = unit_ids.clone();

        join_set.spawn(async move {
            let _permit = permit;
            let addr = SocketAddr::new(IpAddr::V4(ip), port);
            debug!(%addr, "probing host");
            match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr)).await {
                Ok(Ok(_stream)) => {
                    info!(%addr, "discovered modbus host");
                    let mut found = Vec::with_capacity(task_unit_ids.len());
                    for &uid in task_unit_ids.iter() {
                        found.push(DeviceIdentity {
                            ip: ip.to_string(),
                            unit_id: uid,
                        });
                    }
                    Some(found)
                }
                Ok(Err(err)) => {
                    debug!(%addr, error = %err, "connection failed");
                    None
                }
                Err(_) => {
                    warn!(%addr, "connection timed out");
                    None
                }
            }
        });

        if join_set.len() >= config.max_concurrency {
            if let Some(result) = join_set.join_next().await {
                 if let Some(found_list) = result? {
                    devices.extend(found_list);
                 }
            }
        }


        if current == last {
            break;
        }
        current = current.saturating_add(1);
    }

    while let Some(result) = join_set.join_next().await {
        if let Some(found_list) = result? {
            devices.extend(found_list);
        }
    }

    Ok(devices)
}

fn parse_subnet_range(subnet: &str) -> Result<(u32, u32), DiscoveryError> {
    let (ip_part, prefix_part) = subnet
        .split_once('/')
        .ok_or_else(|| DiscoveryError::InvalidSubnet(subnet.to_string()))?;
    let ip: Ipv4Addr = ip_part
        .parse()
        .map_err(|_| DiscoveryError::InvalidSubnet(subnet.to_string()))?;
    let prefix: u8 = prefix_part
        .parse()
        .map_err(|_| DiscoveryError::InvalidSubnet(subnet.to_string()))?;
    if prefix > 32 {
        return Err(DiscoveryError::InvalidSubnet(subnet.to_string()));
    }

    let ip_u32 = ipv4_to_u32(ip);
    let mask = if prefix == 0 {
        0u32
    } else {
        u32::MAX.checked_shl((32 - prefix) as u32).unwrap_or(0)
    };
    let network = ip_u32 & mask;
    let broadcast = network | !mask;

    let (first, last) = if prefix < 31 {
        (network.saturating_add(1), broadcast.saturating_sub(1))
    } else {
        (network, broadcast)
    };

    if first > last {
        return Err(DiscoveryError::InvalidSubnet(subnet.to_string()));
    }

    Ok((first, last))
}

fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from(ip)
}

fn u32_to_ipv4(value: u32) -> Ipv4Addr {
    Ipv4Addr::from(value)
}
