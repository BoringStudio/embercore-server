use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangColor {
    color:       Color,
    name:        String,
    probability: f64,
    tile:        i32,
}
