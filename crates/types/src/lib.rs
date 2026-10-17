use serde::{Deserialize, Serialize};

/// A decoded SunSpec point value: scale factor applied, `Null` when the device marks
/// the point as not implemented.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointValue {
    Null,
    Int(i64),
    Float(f64),
    Text(String),
}

/// Basic identity for an inverter or battery endpoint. Gateways expose several
/// devices on one host, so a device is identified by host *and* unit ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub ip: String,
    pub unit_id: u8,
}

impl DeviceIdentity {
    /// Stable key such as `192.168.1.20/1`, used for logs, metrics and as the Kafka
    /// record key.
    pub fn key(&self) -> String {
        format!("{}/{}", self.ip, self.unit_id)
    }
}

/// One SunSpec model read from one device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    pub device: DeviceIdentity,
    pub model_id: u16,
    pub model_name: String,
    /// Register address of the model header.
    pub start: u16,
    /// Raw registers, including the two header registers.
    pub registers: Vec<u16>,
    pub collected_at_ms: u64,
    /// Decoded points in definition order; empty for models without a definition.
    #[serde(default)]
    pub points: Vec<(String, PointValue)>,
}
