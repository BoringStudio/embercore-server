use super::data_source::DataSource;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Chunk {
    pub data:   DataSource,
    pub height: i32,
    pub width:  i32,
    pub x:      i32,
    pub y:      i32,
}
