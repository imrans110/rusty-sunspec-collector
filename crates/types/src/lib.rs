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
    /// Names of enum values and of set bitfield bits, by point name (see
    /// `sunspec_parser::DecodedModel::labels`).
    #[serde(default)]
    pub labels: Vec<(String, String)>,
}

/// Format version written by `collector-app --probe`.
pub const DEVICE_DUMP_FORMAT: u32 = 1;

/// One device's SunSpec register map, read once: the output of `collector-app --probe`
/// and the fixture format `sunspec-sim --fixture` replays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceDump {
    /// [`DEVICE_DUMP_FORMAT`] when written.
    pub format: u32,
    /// `collector-app` version that wrote the dump.
    pub collector_version: String,
    pub probed_at_ms: u64,
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
    /// Address of the `SunS` marker.
    pub base_address: u16,
    /// Points blanked before writing, as `<model>.<point>` (serial numbers, network
    /// addresses), so dumps can be shared.
    #[serde(default)]
    pub redacted: Vec<String>,
    /// The model chain in device order.
    pub models: Vec<ModelDump>,
}

/// One model of a [`DeviceDump`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelDump {
    pub id: u16,
    pub name: String,
    /// Register address of the model header.
    pub start: u16,
    /// Registers in the model, header included (`L` + 2).
    pub length: u16,
    /// The registers read, header included, as space-separated hex words; empty if the
    /// read failed.
    #[serde(with = "hex_registers")]
    pub registers: Vec<u16>,
    /// Decoded points in definition order.
    #[serde(with = "ordered_points")]
    pub points: Vec<(String, PointValue)>,
    /// Why the model could not be read, if it could not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Registers as `"0001 0042 5275"`: one line per model instead of one per register.
mod hex_registers {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(registers: &[u16], serializer: S) -> Result<S::Ok, S::Error> {
        let words: Vec<String> = registers.iter().map(|word| format!("{word:04x}")).collect();
        serializer.serialize_str(&words.join(" "))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u16>, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.split_whitespace()
            .map(|word| {
                u16::from_str_radix(word, 16)
                    .map_err(|_| de::Error::custom(format!("'{word}' is not a hex register")))
            })
            .collect()
    }
}

/// Name/value pairs (points, labels) as a JSON object that keeps their order; use with
/// `#[serde(with = "types::ordered_points")]`.
pub mod ordered_points {
    use std::fmt;
    use std::marker::PhantomData;

    use serde::de::{MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer, V: Serialize>(
        points: &[(String, V)],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(points.len()))?;
        for (name, value) in points {
            map.serialize_entry(name, value)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>, V: Deserialize<'de>>(
        deserializer: D,
    ) -> Result<Vec<(String, V)>, D::Error> {
        struct Points<V>(PhantomData<V>);

        impl<'de, V: Deserialize<'de>> Visitor<'de> for Points<V> {
            type Value = Vec<(String, V)>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of names to values")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut points = Vec::with_capacity(access.size_hint().unwrap_or(0));
                while let Some(entry) = access.next_entry()? {
                    points.push(entry);
                }
                Ok(points)
            }
        }

        deserializer.deserialize_map(Points(PhantomData))
    }
}
