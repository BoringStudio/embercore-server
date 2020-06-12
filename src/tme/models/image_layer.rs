use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ImageLayer {
    height:            i32,
    id:                i32,
    image:             String,
    name:              String,
    offset_x:          f64,
    offset_y:          f64,
    opacity:           f64,
    properties:        Option<Vec<Property>>,
    start_x:           i32,
    start_y:           i32,
    transparent_color: Option<Color>,
    visible:           bool,
    width:             i32,
    x:                 i32,
    y:                 i32,
}
