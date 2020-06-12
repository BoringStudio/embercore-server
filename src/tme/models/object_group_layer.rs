use serde::Deserialize;
use serde::Serialize;

use super::layer::DrawOrder;
use super::object::Object;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ObjectGroupLayer {
    draw_order: DrawOrder,
    height:     Option<i32>,
    id:         i32,
    name:       String,
    objects:    Vec<Object>,
    offset_x:   Option<f64>,
    offset_y:   Option<f64>,
    opacity:    f64,
    properties: Option<Vec<Property>>,
    start_x:    Option<i32>,
    start_y:    Option<i32>,
    visible:    bool,
    width:      Option<i32>,
    x:          i32,
    y:          i32,
}
