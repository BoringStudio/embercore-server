use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Frame {
    pub duration: i64,
    #[serde(rename = "tiledid")]
    pub tiled_id: i64,
}
