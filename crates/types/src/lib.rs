#![allow(dead_code)]

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

/// Basic identity for an inverter or battery endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub ip: String,
    pub unit_id: u8,
}
