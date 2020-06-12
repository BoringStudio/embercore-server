use serde::Deserialize;
use serde::Deserializer;

pub fn deserialize_value_to_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v: serde_json::Value = Deserialize::deserialize(d)?;
    Ok(v.to_string())
}
