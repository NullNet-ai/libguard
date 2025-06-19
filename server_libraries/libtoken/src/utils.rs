use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn empty_object_or_null_is_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<Value>::deserialize(deserializer)?;

    match opt {
        None => Ok(None),
        Some(Value::Object(map)) if map.is_empty() => Ok(None),
        Some(value) => T::deserialize(value).map(Some).map_err(D::Error::custom),
    }
}
