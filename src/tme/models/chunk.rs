use super::redata::ReData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Chunk {
    data:   ReData,
    height: i32,
    width:  i32,
    x:      i32,
    y:      i32,
}
