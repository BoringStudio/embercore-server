use super::data_source::DataSource;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Chunk {
    data:   DataSource,
    height: i32,
    width:  i32,
    x:      i32,
    y:      i32,
}
