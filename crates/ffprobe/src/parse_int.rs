use serde::{Deserialize, Deserializer};
use std::str::FromStr;

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Value(T),
    }

    match Option::<StringOrInt<T>>::deserialize(deserializer)? {
        Some(StringOrInt::Value(v)) => Ok(Some(v)),
        Some(StringOrInt::String(s)) => {
            if s == "N/A" {
                return Ok(None);
            }
            s.parse::<T>()
                .map(Some)
                .map_err(|_| serde::de::Error::custom("Parse error"))
        }
        None => Ok(None),
    }
}

pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize,
{
    match value {
        Some(v) => v.serialize(serializer),
        None => serializer.serialize_none(),
    }
}