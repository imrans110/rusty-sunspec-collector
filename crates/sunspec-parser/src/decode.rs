use std::collections::HashMap;

use types::PointValue;

use crate::registry::{model_definition, Count, GroupDef, PointDef, PointType, ScaleFactor};

/// Decodes a model's registers (starting at the model header) into named points, in
/// definition order. Returns `None` for models without a bundled definition.
///
/// - Scale factors are applied, so scaled points are `Float`.
/// - Values the device marks as "not implemented" (SunSpec sentinels) are `Null`, as
///   are points whose scale factor is not implemented.
/// - Repeating groups are named `group[i].Point`; fixed nested groups `group.Point`.
/// - The model header (`ID`, `L`), scale factors and padding are omitted.
pub fn decode_model(model_id: u16, registers: &[u16]) -> Option<Vec<(String, PointValue)>> {
    let model = model_definition(model_id)?;
    let mut decoder = Decoder {
        registers,
        pos: 0,
        scopes: Vec::new(),
        out: Vec::new(),
    };
    decoder.group(&model.group, "", true);
    Some(decoder.out)
}

struct Decoder<'a> {
    registers: &'a [u16],
    pos: usize,
    /// Scale-factor values visible from the group being decoded, innermost last.
    /// `None` means the scale factor is not implemented.
    scopes: Vec<HashMap<&'a str, Option<i16>>>,
    out: Vec<(String, PointValue)>,
}

impl<'a> Decoder<'a> {
    fn group(&mut self, group: &'a GroupDef, prefix: &str, top_level: bool) {
        // Scale factors may follow the points that use them, so slice the whole
        // group first, then compute values.
        let mut scope = HashMap::new();
        let mut counts = HashMap::new();
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
                    scope.insert(point.name.as_str(), (sf != i16::MIN).then_some(sf));
                }
                PointType::Count | PointType::Uint16 => {
                    counts.insert(point.name.as_str(), raw[0]);
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
                self.out.push((format!("{prefix}{}", point.name), value));
            }
        }

        for nested in &group.groups {
            let size = nested.fixed_size();
            let repetitions = match &nested.count {
                Count::Fixed(0) if size > 0 => (self.registers.len() - self.pos) / size,
                Count::Fixed(0) => 0,
                Count::Fixed(count) => usize::from(*count),
                Count::Point(name) => counts.get(name.as_str()).copied().map_or(0, usize::from),
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
            (Numeric::Int(value), Some(exponent)) => {
                PointValue::Float(scale(value as f64, exponent))
            }
            (Numeric::Float(value), None) => PointValue::Float(value),
            (Numeric::Float(value), Some(exponent)) => PointValue::Float(scale(value, exponent)),
        }
    }

    fn scale_factor(&self, name: &str) -> Option<Option<i16>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
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

fn scale(value: f64, exponent: i16) -> f64 {
    // Round away float noise from the power of ten (e.g. 2301 * 0.1).
    let scaled = value * 10f64.powi(i32::from(exponent));
    if exponent < 0 {
        let factor = 10f64.powi(i32::from(-exponent));
        (scaled * factor).round() / factor
    } else {
        scaled
    }
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
