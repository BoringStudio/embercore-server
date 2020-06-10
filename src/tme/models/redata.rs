use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReData {
    Raw(Vec<i32>),
    Encoded(String),
}
