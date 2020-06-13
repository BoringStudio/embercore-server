use serde::Deserialize;
use serde::Serialize;

use super::layer::DrawOrder;
use super::object::Object;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ObjectGroupLayer {
    pub draw_order: DrawOrder,
    pub height:     Option<i32>,
    pub id:         i32,
    pub name:       String,
    pub objects:    Vec<Object>,
    pub offset_x:   Option<f64>,
    pub offset_y:   Option<f64>,
    pub opacity:    f64,
    pub properties: Option<Vec<Property>>,
    pub start_x:    Option<i32>,
    pub start_y:    Option<i32>,
    pub visible:    bool,
    pub width:      Option<i32>,
    pub x:          i32,
    pub y:          i32,
}
