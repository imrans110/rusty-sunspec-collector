//! SunSpec model definitions and decoding.
//!
//! Model definitions are the official SunSpec JSON files (see `models/`), embedded at
//! compile time. [`decode_model`] turns a model's raw registers into named points with
//! scale factors applied and "not implemented" values mapped to null.

mod decode;
mod registry;

pub use decode::decode_model;
pub use registry::{
    model_definition, supported_models, Count, GroupDef, ModelDef, PointDef, PointType, ScaleFactor,
};

/// The two registers ("SunS") that mark the start of a SunSpec register map.
pub const SUNSPEC_ID: [u16; 2] = [0x5375, 0x6e53];
/// Model ID that terminates the model chain.
pub const SUNSPEC_END_ID: u16 = 0xFFFF;

/// A model found on a device by walking its model chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDefinition {
    pub id: u16,
    pub name: String,
    /// Register start address for this model.
    pub start: u16,
    /// Total register count including the model header (ID + length).
    pub length: u16,
}

/// The SunSpec name of a model (e.g. `inverter_three_phase`), or `model_<id>` for
/// models without a bundled definition.
pub fn model_name(model_id: u16) -> String {
    model_definition(model_id)
        .map(|model| model.group.name.clone())
        .unwrap_or_else(|| format!("model_{model_id}"))
}
