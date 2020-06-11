use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Text {
    bold:        bool,
    color:       Color,
    font_family: String,
    h_align:     HorizontalAlign,
    italic:      bool,
    kerning:     bool,
    pixel_size:  i32,
    strike_out:  bool,
    text:        String,
    underline:   bool,
    v_align:     VerticalAlign,
    wrap:        bool,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlign {
    Center,
    Right,
    Justify,
    Left,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    Center,
    Bottom,
    Top,
}
