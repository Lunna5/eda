use serde::{Deserialize, Deserializer};
use serde::de::Error;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        String(String),
        Float(f64),
        Int(i64),
    }

    match Option::<StringOrFloat>::deserialize(deserializer)? {
        Some(StringOrFloat::Float(f)) => Ok(Some(f)),
        Some(StringOrFloat::Int(i)) => Ok(Some(i as f64)),
        Some(StringOrFloat::String(s)) => {
            if s == "N/A" {
                return Ok(None);
            }
            s.parse::<f64>()
                .map(Some)
                .map_err(|_| Error::custom(format!("Failed to parse float from: {}", s)))
        }
        None => Ok(None),
    }
}

pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => serializer.serialize_f64(*v),
        None => serializer.serialize_none(),
    }
}