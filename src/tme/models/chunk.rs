use super::data_source::DataSource;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Chunk {
    pub data:   DataSource,
    pub height: i64,
    pub width:  i64,
    pub x:      i64,
    pub y:      i64,
}
