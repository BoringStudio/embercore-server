use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangColor {
    pub color:       Color,
    pub name:        String,
    pub probability: f64,
    pub tile:        i64,
}
