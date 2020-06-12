use serde::Deserialize;
use serde::Serialize;

use super::chunk::Chunk;
use super::data_source::DataSource;
use super::layer::Compression;
use super::layer::Encoding;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct TileLayer {
    chunks:      Vec<Chunk>,
    compression: Compression,
    data:        DataSource,
    encoding:    Encoding,
    height:      i32,
    id:          i32,
    name:        String,
    offset_x:    f64,
    offset_y:    f64,
    opacity:     f64,
    properties:  Option<Vec<Property>>,
    start_x:     i32,
    start_y:     i32,
    visible:     bool,
    width:       i32,
    x:           i32,
    y:           i32,
}
