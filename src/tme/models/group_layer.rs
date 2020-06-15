use serde::Deserialize;
use serde::Serialize;

use super::layer::Layer;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct GroupLayer {
    pub id:         i64,
    pub layers:     Vec<Layer>,
    pub name:       String,
    #[serde(rename = "offsetx")]
    pub offset_x:   Option<f64>,
    #[serde(rename = "offsety")]
    pub offset_y:   Option<f64>,
    pub opacity:    f64,
    pub properties: Option<Vec<Property>>,
    #[serde(rename = "startx")]
    pub start_x:    Option<i64>,
    #[serde(rename = "starty")]
    pub start_y:    Option<i64>,
    pub visible:    bool,
    pub x:          i64,
    pub y:          i64,
}
