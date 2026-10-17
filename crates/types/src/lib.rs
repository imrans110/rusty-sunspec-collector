use serde::{Deserialize, Serialize};

/// Raw point values before SunSpec scale factors are applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointValue {
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    F32(f32),
}

/// Basic identity for an inverter or battery endpoint. Gateways expose several
/// devices on one host, so a device is identified by host *and* unit ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub ip: String,
    pub unit_id: u8,
}

impl DeviceIdentity {
    /// Stable key such as `192.168.1.20/1`, used for logs, metrics and partitioning.
    pub fn key(&self) -> String {
        format!("{}/{}", self.ip, self.unit_id)
    }
}
