use serde::Deserialize;
use serde::Serialize;

use super::point::Point;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PolylineObject {
    height:     f64,
    id:         i32,
    name:       String,
    properties: Option<Vec<Property>>,
    polyline:   Vec<Point>,
    rotation:   f64,
    template:   Option<String>,
    #[serde(rename = "type")]
    obj_type:   String,
    visible:    bool,
    width:      f64,
    x:          f64,
    y:          f64,
}
