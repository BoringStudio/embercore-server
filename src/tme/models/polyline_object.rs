use serde::Deserialize;
use serde::Serialize;

use super::point::Point;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PolylineObject {
    pub height:     f64,
    pub id:         i32,
    pub name:       String,
    pub properties: Option<Vec<Property>>,
    pub polyline:   Vec<Point>,
    pub rotation:   f64,
    pub template:   Option<String>,
    #[serde(rename = "type")]
    pub obj_type:   String,
    pub visible:    bool,
    pub width:      f64,
    pub x:          f64,
    pub y:          f64,
}
