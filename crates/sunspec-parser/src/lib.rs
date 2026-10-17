use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use serde::Deserialize;
use thiserror::Error;
use tracing::warn;
use types::PointValue;

#[derive(Debug, Clone)]
pub struct ModelDefinition {
    pub id: u16,
    pub name: String,
    /// Register start address for this model.
    pub start: u16,
    /// Total register count including the model header (ID + length).
    pub length: u16,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("xml parse error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("invalid attribute value for {0}")]
    InvalidAttribute(String),
}

/// The two registers ("SunS") that mark the start of a SunSpec register map.
pub const SUNSPEC_ID: [u16; 2] = [0x5375, 0x6e53];
/// Model ID that terminates the model chain.
pub const SUNSPEC_END_ID: u16 = 0xFFFF;

#[derive(Default)]
pub struct ModelCatalog {
    json_cache: HashMap<u64, Vec<ModelDefinition>>,
    xml_cache: HashMap<u64, Vec<ModelDefinition>>,
}

impl ModelCatalog {
    pub fn parse_json(&mut self, data: &str) -> Result<Vec<ModelDefinition>, ParserError> {
        let key = fingerprint(data);
        if let Some(models) = self.json_cache.get(&key) {
            return Ok(models.clone());
        }
        let models = parse_models_from_json(data)?;
        self.json_cache.insert(key, models.clone());
        Ok(models)
    }

    pub fn parse_xml(&mut self, data: &str) -> Result<Vec<ModelDefinition>, ParserError> {
        let key = fingerprint(data);
        if let Some(models) = self.xml_cache.get(&key) {
            return Ok(models.clone());
        }
        let models = parse_models_from_xml(data)?;
        self.xml_cache.insert(key, models.clone());
        Ok(models)
    }

    pub fn json_cache_len(&self) -> usize {
        self.json_cache.len()
    }

    pub fn xml_cache_len(&self) -> usize {
        self.xml_cache.len()
    }
}

#[derive(Debug, Deserialize)]
struct JsonModel {
    id: u16,
    name: String,
    #[serde(alias = "len", alias = "length")]
    length: u16,
}

#[derive(Debug, Deserialize)]
struct JsonRoot {
    models: Vec<JsonModel>,
}

pub fn parse_models_from_json(data: &str) -> Result<Vec<ModelDefinition>, ParserError> {
    if let Ok(models) = serde_json::from_str::<Vec<JsonModel>>(data) {
        return Ok(models
            .into_iter()
            .map(|model| ModelDefinition {
                id: model.id,
                name: model.name,
                start: 0,
                length: model.length.saturating_add(2),
            })
            .collect());
    }

    let root: JsonRoot = serde_json::from_str(data)?;
    Ok(root
        .models
        .into_iter()
        .map(|model| ModelDefinition {
            id: model.id,
            name: model.name,
            start: 0,
            length: model.length.saturating_add(2),
        })
        .collect())
}

pub fn parse_models_from_xml(data: &str) -> Result<Vec<ModelDefinition>, ParserError> {
    let mut reader = Reader::from_str(data);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut models = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref event) | Event::Empty(ref event))
                if event.name().as_ref() == "model" =>
            {
                let mut id = None;
                let mut name = None;
                let mut length = None;

                for attr in event.attributes() {
                    let attr = attr.map_err(|e| ParserError::InvalidAttribute(e.to_string()))?;
                    let key = attr.key.as_ref();
                    let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();

                    match key {
                        "id" => {
                            id =
                                Some(value.parse::<u16>().map_err(|_| {
                                    ParserError::InvalidAttribute("id".to_string())
                                })?);
                        }
                        "name" => {
                            name = Some(value);
                        }
                        "len" | "length" => {
                            length = Some(value.parse::<u16>().map_err(|_| {
                                ParserError::InvalidAttribute("length".to_string())
                            })?);
                        }
                        _ => {}
                    }
                }

                if let (Some(id), Some(length)) = (id, length) {
                    let name = name.unwrap_or_else(|| format!("model_{id}"));
                    models.push(ModelDefinition {
                        id,
                        name,
                        start: 0,
                        length: length.saturating_add(2),
                    });
                } else {
                    warn!("skipping model with missing id or length");
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => return Err(ParserError::Xml(err)),
        }

        buf.clear();
    }

    Ok(models)
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

/// Human-readable name for well-known SunSpec model IDs.
pub fn model_name(model_id: u16) -> String {
    match model_id {
        1 => "common".to_string(),
        101 => "inverter".to_string(),
        103 => "three_phase_inverter".to_string(),
        160 => "mppt".to_string(),
        201 => "meter".to_string(),
        _ => format!("model_{model_id}"),
    }
}

fn fingerprint(value: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
