use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct ImageLayer {
    pub id:                i64,
    pub image:             String,
    pub name:              String,
    #[serde(rename = "offsetx")]
    pub offset_x:          Option<f64>,
    #[serde(rename = "offsety")]
    pub offset_y:          Option<f64>,
    pub opacity:           f64,
    pub properties:        Option<Vec<Property>>,
    #[serde(rename = "startx")]
    pub start_x:           Option<i64>,
    #[serde(rename = "starty")]
    pub start_y:           Option<i64>,
    #[serde(rename = "transparentcolor")]
    pub transparent_color: Option<Color>,
    pub visible:           bool,
    pub x:                 i64,
    pub y:                 i64,
}
