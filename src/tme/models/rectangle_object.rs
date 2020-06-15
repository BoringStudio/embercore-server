use serde::Deserialize;
use serde::Serialize;

use super::property::Property;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct RectangleObject {
    pub height:     f64,
    pub id:         i64,
    pub name:       String,
    pub properties: Option<Vec<Property>>,
    pub rotation:   f64,
    pub template:   Option<String>,
    #[serde(rename = "type")]
    pub obj_type:   String,
    pub visible:    bool,
    pub width:      f64,
    pub x:          f64,
    pub y:          f64,
}
