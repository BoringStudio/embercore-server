use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataSource {
    Array(Vec<i32>),
    Base64(String),
}
