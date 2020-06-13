use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Text {
    pub bold:        bool,
    pub color:       Color,
    pub font_family: String,
    pub h_align:     HorizontalAlign,
    pub italic:      bool,
    pub kerning:     bool,
    pub pixel_size:  i32,
    pub strike_out:  bool,
    pub text:        String,
    pub underline:   bool,
    pub v_align:     VerticalAlign,
    pub wrap:        bool,
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
