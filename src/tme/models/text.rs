use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Text {
    pub bold:        bool,
    #[serde(with = "color_serde")]
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

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_terrain() {
        let actuals: Vec<Text> = serde_json::from_value(json! {
            [
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "center",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "center",
                    "wrap":       true
                },
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "right",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "bottom",
                    "wrap":       true
                },
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "justify",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                },
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "left",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Text> = vec![
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Center,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Center,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Right,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Bottom,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Justify,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Left,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_terrain() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "center",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "center",
                    "wrap":       true
                }
            },
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "right",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "bottom",
                    "wrap":       true
                }
            },
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "justify",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                }
            },
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "left",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Center,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Center,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Right,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Bottom,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Justify,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Left,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
