//! Metadata for every bundled model (labels, descriptions, units, types and symbols), so
//! consumers can describe decoded points without their own copy of the definitions.

use serde::Serialize;

use crate::registry::{model_definition, supported_models, Count, GroupDef, PointType};

/// One model's metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModelInfo {
    pub id: u16,
    /// As in records' `model_name`.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The points records carry, in definition order (the header, scale factors and
    /// padding are left out, as in records).
    pub points: Vec<PointInfo>,
}

/// One point's metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PointInfo {
    /// The name records use; a repeating group's index is `[*]`, so `module[*].DCA`
    /// describes `module[0].DCA`, `module[1].DCA` and so on.
    pub name: String,
    #[serde(rename = "type")]
    pub kind: PointType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Enum values, or bitfield bit positions, and their names.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SymbolInfo {
    pub name: String,
    pub value: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Metadata for one bundled model.
pub fn model_info(model_id: u16) -> Option<ModelInfo> {
    let model = model_definition(model_id)?;
    let mut points = Vec::new();
    collect(&model.group, "", true, &mut points);
    Some(ModelInfo {
        id: model.id,
        name: crate::model_name(model.id),
        label: model.group.label.clone(),
        description: model.group.desc.clone(),
        points,
    })
}

/// Metadata for every bundled model, by ascending ID.
pub fn catalog() -> Vec<ModelInfo> {
    supported_models()
        .into_iter()
        .filter_map(model_info)
        .collect()
}

fn collect(group: &GroupDef, prefix: &str, top_level: bool, out: &mut Vec<PointInfo>) {
    for point in &group.points {
        let omitted = matches!(point.kind, PointType::Pad | PointType::Sunssf)
            || (top_level && (point.name == "ID" || point.name == "L"));
        if omitted {
            continue;
        }
        out.push(PointInfo {
            name: format!("{prefix}{}", point.name),
            kind: point.kind,
            units: point.units.clone(),
            label: point.label.clone(),
            description: point.desc.clone(),
            symbols: point
                .symbols
                .iter()
                .map(|symbol| SymbolInfo {
                    name: symbol.name.clone(),
                    value: symbol.value,
                    label: symbol.label.clone(),
                    description: symbol.desc.clone(),
                })
                .collect(),
        });
    }
    for nested in &group.groups {
        let nested_prefix = if matches!(nested.count, Count::Fixed(1)) {
            format!("{prefix}{}.", nested.name)
        } else {
            format!("{prefix}{}[*].", nested.name)
        };
        collect(nested, &nested_prefix, false, out);
    }
}
