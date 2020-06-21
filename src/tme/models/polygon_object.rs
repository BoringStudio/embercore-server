use serde::Deserialize;
use serde::Serialize;

use super::point::Point;
use super::property::Property;
use super::utils;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct PolygonObject {
    pub height:     f64,
    pub id:         i64,
    pub name:       String,
    #[serde(default = "utils::make_none_option")]
    pub properties: Option<Vec<Property>>,
    pub polygon:    Vec<Point>,
    pub rotation:   f64,
    #[serde(default = "utils::make_none_option")]
    pub template:   Option<String>,
    #[serde(rename = "type")]
    pub obj_type:   String,
    pub visible:    bool,
    pub width:      f64,
    pub x:          f64,
    pub y:          f64,
}
