use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Frame {
    pub duration: i32,
    pub tiled_id: i32,
}
