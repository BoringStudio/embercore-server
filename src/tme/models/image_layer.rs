use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ImageLayer {
    pub height:            i32,
    pub id:                i32,
    pub image:             String,
    pub name:              String,
    pub offset_x:          f64,
    pub offset_y:          f64,
    pub opacity:           f64,
    pub properties:        Option<Vec<Property>>,
    pub start_x:           i32,
    pub start_y:           i32,
    pub transparent_color: Option<Color>,
    pub visible:           bool,
    pub width:             i32,
    pub x:                 i32,
    pub y:                 i32,
}
