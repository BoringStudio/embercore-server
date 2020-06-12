use serde::Deserialize;
use serde::Serialize;

use super::layer::Layer;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct GroupLayer {
    height:     i32,
    id:         i32,
    layers:     Vec<Layer>,
    name:       String,
    offset_x:   f64,
    offset_y:   f64,
    opacity:    f64,
    properties: Option<Vec<Property>>,
    start_x:    i32,
    start_y:    i32,
    visible:    bool,
    width:      i32,
    x:          i32,
    y:          i32,
}
