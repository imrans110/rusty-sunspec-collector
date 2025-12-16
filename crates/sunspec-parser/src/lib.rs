#![allow(dead_code)]

use thiserror::Error;
use types::PointValue;

#[derive(Debug, Clone)]
pub struct ModelDefinition {
    pub id: u16,
    pub name: String,
    pub length: u16,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("parsing not implemented")]
    NotImplemented,
}

pub fn parse_models_from_json(_data: &str) -> Result<Vec<ModelDefinition>, ParserError> {
    // TODO: load SunSpec JSON/XML definitions and build an address map.
    Err(ParserError::NotImplemented)
}

/// SunSpec marks absent values with sentinel patterns (e.g., 0x8000 for i16). Returns None when the raw value is a sentinel.
pub fn apply_scale(raw: PointValue, scale_factor: i16) -> Option<f64> {
    match raw {
        PointValue::I16(v) if v == i16::MIN => None,
        PointValue::U16(v) if v == u16::MAX => None,
        PointValue::I32(v) if v == i32::MIN => None,
        PointValue::U32(v) if v == u32::MAX => None,
        PointValue::F32(v) if v.is_nan() => None,
        PointValue::I16(v) => Some((v as f64) * 10f64.powi(scale_factor as i32)),
        PointValue::U16(v) => Some((v as f64) * 10f64.powi(scale_factor as i32)),
        PointValue::I32(v) => Some((v as f64) * 10f64.powi(scale_factor as i32)),
        PointValue::U32(v) => Some((v as f64) * 10f64.powi(scale_factor as i32)),
        PointValue::F32(v) => Some((v as f64) * 10f64.powi(scale_factor as i32)),
    }
}
