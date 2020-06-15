use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Text {
    pub bold:        bool,
    pub color:       Color,
    #[serde(rename = "fontfamily")]
    pub font_family: String,
    #[serde(rename = "halign")]
    pub h_align:     HorizontalAlign,
    pub italic:      bool,
    pub kerning:     bool,
    #[serde(rename = "pixelsize")]
    pub pixel_size:  i64,
    #[serde(rename = "strikeout")]
    pub strike_out:  bool,
    pub text:        String,
    pub underline:   bool,
    #[serde(rename = "valign")]
    pub v_align:     VerticalAlign,
    pub wrap:        bool,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlign {
    Center,
    Right,
    Justify,
    Left,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    Center,
    Bottom,
    Top,
}
