use std::collections::HashMap;

use types::PointValue;

use crate::registry::{model_definition, Count, GroupDef, PointDef, PointType, ScaleFactor};

/// Decodes a model's registers (starting at the model header) into named points, in
/// definition order. Returns `None` for models without a bundled definition.
///
/// - Scale factors are applied, so scaled points are `Float`.
/// - Values the device marks as "not implemented" (SunSpec sentinels) are `Null`, as
///   are points whose scale factor is not implemented.
/// - Values that are not a finite number are `Null`: an infinite float, or a value
///   scaled by a factor far outside SunSpec's -10 to 10.
/// - Repeating groups are named `group[i].Point`; fixed nested groups `group.Point`.
/// - The model header (`ID`, `L`), scale factors and padding are omitted.
pub fn decode_model(model_id: u16, registers: &[u16]) -> Option<Vec<(String, PointValue)>> {
    decode_model_labeled(model_id, registers).map(|decoded| decoded.points)
}

/// A decoded model: its points, and names for its enum and bitfield values.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecodedModel {
    /// As returned by [`decode_model`].
    pub points: Vec<(String, PointValue)>,
    /// For each implemented enum point, the symbol name of its value (when the
    /// definition names it); for each implemented bitfield, the names of the set bits,
    /// comma-separated, with `bit<n>` for unnamed bits, and an empty string when no bit
    /// is set. In definition order.
    pub labels: Vec<(String, String)>,
}

/// [`decode_model`] plus enum and bitfield names, in one pass.
pub fn decode_model_labeled(model_id: u16, registers: &[u16]) -> Option<DecodedModel> {
    let model = model_definition(model_id)?;
    let mut decoder = Decoder {
        registers,
        pos: 0,
        scopes: Vec::new(),
        out: Vec::new(),
        labels: Vec::new(),
    };
    decoder.group(&model.group, "", true);
    Some(DecodedModel {
        points: decoder.out,
        labels: decoder.labels,
    })
}

/// The name of an enum value, or the names of a bitfield's set bits.
fn label(point: &PointDef, value: &PointValue) -> Option<String> {
    let PointValue::Int(value) = *value else {
        return None;
    };
    match point.kind {
        PointType::Enum16 | PointType::Enum32 => point
            .symbols
            .iter()
            .find(|symbol| symbol.value == value)
            .map(|symbol| symbol.name.clone()),
        PointType::Bitfield16 | PointType::Bitfield32 => {
            let names: Vec<String> = (0..64)
                .filter(|bit| value >> bit & 1 == 1)
                .map(|bit| {
                    point
                        .symbols
                        .iter()
                        .find(|symbol| symbol.value == bit)
                        .map_or_else(|| format!("bit{bit}"), |symbol| symbol.name.clone())
                })
                .collect();
            Some(names.join(","))
        }
        _ => None,
    }
}

struct Decoder<'a> {
    registers: &'a [u16],
    pos: usize,
    /// Values visible from the group being decoded, innermost group last.
    scopes: Vec<Scope<'a>>,
    out: Vec<(String, PointValue)>,
    labels: Vec<(String, String)>,
}

/// Points of one group instance that other points or groups refer to by name.
#[derive(Default)]
struct Scope<'a> {
    /// Scale factors; `None` means the scale factor is not implemented.
    scale_factors: HashMap<&'a str, Option<i16>>,
    /// Integer points that may give a nested group's repetition count (e.g. `NPt`).
    counts: HashMap<&'a str, u16>,
}

impl<'a> Decoder<'a> {
    fn group(&mut self, group: &'a GroupDef, prefix: &str, top_level: bool) {
        // Scale factors may follow the points that use them, so slice the whole
        // group first, then compute values.
        let mut scope = Scope::default();
        let mut raws = Vec::with_capacity(group.points.len());
        for point in &group.points {
            let size = usize::from(point.size);
            let Some(raw) = self.registers.get(self.pos..self.pos + size) else {
                self.pos = self.registers.len();
                break;
            };
            self.pos += size;
            match point.kind {
                PointType::Sunssf => {
                    let sf = raw[0] as i16;
                    scope
                        .scale_factors
                        .insert(point.name.as_str(), (sf != i16::MIN).then_some(sf));
                }
                PointType::Count | PointType::Uint16 => {
                    scope.counts.insert(point.name.as_str(), raw[0]);
                }
                _ => {}
            }
            raws.push((point, raw));
        }
        self.scopes.push(scope);

        for (point, raw) in raws {
            let skip = matches!(point.kind, PointType::Pad | PointType::Sunssf)
                || (top_level && (point.name == "ID" || point.name == "L"));
            if !skip {
                let value = self.value(point, raw);
                let name = format!("{prefix}{}", point.name);
                if let Some(label) = label(point, &value) {
                    self.labels.push((name.clone(), label));
                }
                self.out.push((name, value));
            }
        }

        for nested in &group.groups {
            let size = nested.fixed_size();
            let repetitions = match &nested.count {
                Count::Fixed(0) if size > 0 => (self.registers.len() - self.pos) / size,
                Count::Fixed(0) => 0,
                Count::Fixed(count) => usize::from(*count),
                // The count point may live in an enclosing group (e.g. the curve models'
                // top-level `NPt` sizes `Crv[i].Pt`). 0xFFFF means not implemented.
                Count::Point(name) => match self.count(name) {
                    Some(count) if count != u16::MAX => usize::from(count),
                    _ => 0,
                },
            };
            let repeating = !matches!(nested.count, Count::Fixed(1));
            for index in 0..repetitions {
                if self.pos >= self.registers.len() {
                    break;
                }
                let nested_prefix = if repeating {
                    format!("{prefix}{}[{index}].", nested.name)
                } else {
                    format!("{prefix}{}.", nested.name)
                };
                self.group(nested, &nested_prefix, false);
            }
        }
        self.scopes.pop();
    }

    fn value(&self, point: &PointDef, raw: &[u16]) -> PointValue {
        let numeric = match raw_value(point.kind, raw) {
            Raw::Null => return PointValue::Null,
            Raw::Text(text) => return PointValue::Text(text),
            Raw::Int(value) => Numeric::Int(value),
            Raw::Float(value) => Numeric::Float(value),
        };
        let exponent = match &point.sf {
            None => None,
            Some(ScaleFactor::Fixed(exponent)) => Some(*exponent),
            Some(ScaleFactor::Point(name)) => match self.scale_factor(name) {
                Some(Some(exponent)) => Some(exponent),
                // The scale factor is "not implemented", so the value is unusable.
                Some(None) => return PointValue::Null,
                None => None,
            },
        };
        match (numeric, exponent) {
            (Numeric::Int(value), None) => match i64::try_from(value) {
                Ok(value) => PointValue::Int(value),
                Err(_) => PointValue::Float(value as f64),
            },
            (Numeric::Int(value), Some(exponent)) => finite(scale(value as f64, exponent)),
            (Numeric::Float(value), None) => finite(value),
            (Numeric::Float(value), Some(exponent)) => finite(scale(value, exponent)),
        }
    }

    fn scale_factor(&self, name: &str) -> Option<Option<i16>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.scale_factors.get(name).copied())
    }

    fn count(&self, name: &str) -> Option<u16> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.counts.get(name).copied())
    }
}

enum Raw {
    Null,
    Int(i128),
    Float(f64),
    Text(String),
}

enum Numeric {
    Int(i128),
    Float(f64),
}

/// A float point, or `Null` when the value is no reading (infinite or NaN).
fn finite(value: f64) -> PointValue {
    if value.is_finite() {
        PointValue::Float(value)
    } else {
        PointValue::Null
    }
}

fn scale(value: f64, exponent: i16) -> f64 {
    // Round away float noise from the power of ten (e.g. 2301 * 0.1). Past about 1e308
    // the factor is infinite and rounding would give NaN, so such values stay unrounded.
    let scaled = value * 10f64.powi(i32::from(exponent));
    if exponent < 0 {
        let factor = 10f64.powi(-i32::from(exponent));
        if factor.is_finite() {
            return (scaled * factor).round() / factor;
        }
    }
    scaled
}

fn wide(raw: &[u16]) -> u64 {
    raw.iter()
        .fold(0u64, |acc, register| (acc << 16) | u64::from(*register))
}

/// Applies the SunSpec "not implemented" sentinel for each type.
fn raw_value(kind: PointType, raw: &[u16]) -> Raw {
    let unsigned = wide(raw);
    match kind {
        PointType::Int16 | PointType::Sunssf => {
            let value = raw[0] as i16;
            if value == i16::MIN {
                Raw::Null
            } else {
                Raw::Int(value.into())
            }
        }
        PointType::Uint16 | PointType::Enum16 | PointType::Bitfield16 => {
            if raw[0] == u16::MAX {
                Raw::Null
            } else {
                Raw::Int(raw[0].into())
            }
        }
        PointType::Count | PointType::Acc16 => {
            if raw[0] == 0 {
                Raw::Null
            } else {
                Raw::Int(raw[0].into())
            }
        }
        PointType::Int32 => {
            let value = unsigned as u32 as i32;
            if value == i32::MIN {
                Raw::Null
            } else {
                Raw::Int(value.into())
            }
        }
        PointType::Uint32 | PointType::Enum32 | PointType::Bitfield32 => {
            if unsigned == u64::from(u32::MAX) {
                Raw::Null
            } else {
                Raw::Int(unsigned.into())
            }
        }
        PointType::Acc32 | PointType::Ipaddr | PointType::Acc64 => {
            if unsigned == 0 {
                Raw::Null
            } else {
                Raw::Int(unsigned.into())
            }
        }
        PointType::Int64 => {
            let value = unsigned as i64;
            if value == i64::MIN {
                Raw::Null
            } else {
                Raw::Int(value.into())
            }
        }
        PointType::Uint64 => {
            if unsigned == u64::MAX {
                Raw::Null
            } else {
                Raw::Int(unsigned.into())
            }
        }
        PointType::Float32 => {
            let value = f32::from_bits(unsigned as u32);
            if value.is_nan() {
                Raw::Null
            } else {
                Raw::Float(value.into())
            }
        }
        PointType::Float64 => {
            let value = f64::from_bits(unsigned);
            if value.is_nan() {
                Raw::Null
            } else {
                Raw::Float(value)
            }
        }
        PointType::String => {
            let bytes: Vec<u8> = raw
                .iter()
                .flat_map(|register| register.to_be_bytes())
                .collect();
            let text = String::from_utf8_lossy(&bytes);
            let text = text.trim_matches(|c: char| c == '\0' || c.is_whitespace());
            if text.is_empty() {
                Raw::Null
            } else {
                Raw::Text(text.to_string())
            }
        }
        PointType::Eui48 | PointType::Ipv6addr => {
            let empty = if kind == PointType::Eui48 {
                u16::MAX
            } else {
                0
            };
            if raw.iter().all(|register| *register == empty) {
                Raw::Null
            } else {
                Raw::Text(
                    raw.iter()
                        .map(|register| format!("{register:04x}"))
                        .collect(),
                )
            }
        }
        PointType::Pad | PointType::Unknown => Raw::Null,
    }
}
