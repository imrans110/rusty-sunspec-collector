use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;

/// Official SunSpec model definitions (github.com/sunspec/models, Apache-2.0): every
/// `models/model_<id>.json`, embedded by `build.rs`.
const MODEL_SOURCES: &[&str] = include!(concat!(env!("OUT_DIR"), "/model_sources.rs"));

#[derive(Debug, Clone, Deserialize)]
pub struct ModelDef {
    pub id: u16,
    pub group: GroupDef,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupDef {
    pub name: String,
    /// Repetitions: a fixed number (0 = repeat to fill the model) or a point holding it.
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub points: Vec<PointDef>,
    #[serde(default)]
    pub groups: Vec<GroupDef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Count {
    Fixed(u16),
    Point(String),
}

impl Default for Count {
    fn default() -> Self {
        Count::Fixed(1)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointDef {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: PointType,
    #[serde(default = "one")]
    pub size: u16,
    #[serde(default)]
    pub sf: Option<ScaleFactor>,
    #[serde(default)]
    pub units: Option<String>,
}

fn one() -> u16 {
    1
}

/// A scale factor: a literal exponent, or the name of a `sunssf` point.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum ScaleFactor {
    Fixed(i16),
    Point(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PointType {
    Int16,
    Uint16,
    Count,
    Acc16,
    Enum16,
    Bitfield16,
    Pad,
    Sunssf,
    Int32,
    Uint32,
    Acc32,
    Enum32,
    Bitfield32,
    Ipaddr,
    Int64,
    Uint64,
    Acc64,
    Float32,
    Float64,
    String,
    Ipv6addr,
    Eui48,
    #[serde(other)]
    Unknown,
}

impl GroupDef {
    /// Registers in one repetition of this group, counting nested groups with a
    /// fixed count (repeat-to-fill nested groups contribute nothing).
    pub fn fixed_size(&self) -> usize {
        let points: usize = self
            .points
            .iter()
            .map(|point| usize::from(point.size))
            .sum();
        let groups: usize = self
            .groups
            .iter()
            .map(|group| match group.count {
                Count::Fixed(count) => usize::from(count) * group.fixed_size(),
                Count::Point(_) => 0,
            })
            .sum();
        points + groups
    }
}

impl ModelDef {
    /// Register offset (from the model header) and definition of a top-level point.
    pub fn point_offset(&self, name: &str) -> Option<(u16, &PointDef)> {
        let mut offset = 0u16;
        for point in &self.group.points {
            if point.name == name {
                return Some((offset, point));
            }
            offset += point.size;
        }
        None
    }
}

fn registry() -> &'static HashMap<u16, ModelDef> {
    static REGISTRY: OnceLock<HashMap<u16, ModelDef>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        MODEL_SOURCES
            .iter()
            .map(|source| {
                let model: ModelDef =
                    serde_json::from_str(source).expect("bundled SunSpec model JSON is valid");
                (model.id, model)
            })
            .collect()
    })
}

/// The bundled definition of a SunSpec model, if there is one.
pub fn model_definition(model_id: u16) -> Option<&'static ModelDef> {
    registry().get(&model_id)
}

/// IDs of every model with a bundled definition, ascending.
pub fn supported_models() -> Vec<u16> {
    let mut ids: Vec<u16> = registry().keys().copied().collect();
    ids.sort_unstable();
    ids
}
