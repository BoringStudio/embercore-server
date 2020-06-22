use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use super::text::Text;
use super::utils;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct TextObject {
    pub height:     f64,
    pub id:         i64,
    pub name:       String,
    #[serde(default = "utils::make_none_option")]
    pub properties: Option<Vec<Property>>,
    pub rotation:   f64,
    #[serde(default = "utils::make_none_option")]
    pub template:   Option<String>,
    pub text:       Text,
    #[serde(rename = "type")]
    pub obj_type:   String,
    pub visible:    bool,
    pub width:      f64,
    pub x:          f64,
    pub y:          f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::color::Color;
    use crate::tme::models::text::{HorizontalAlign, VerticalAlign};
    use serde_json::json;

    #[test]
    fn deserialize_text_object() {
        let actuals: Vec<TextObject> = serde_json::from_value(json! {
            [
                {
                    "height":     1111.8,
                    "id":         42,
                    "name":       "Magica",
                    "properties": null,
                    "rotation":   77.77,
                    "template":   null,
                    "text":       {
                        "bold":       false,
                        "color":      "#FF00FFAA",
                        "fontfamily": "arial",
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
                    "type":       "npc",
                    "visible":    true,
                    "width":      7777.7,
                    "x":          6.7,
                    "y":          7.6
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<TextObject> = vec![TextObject {
            height:     1111.8,
            id:         42,
            name:       "Magica".to_string(),
            properties: None,
            rotation:   77.77,
            template:   None,
            text:       Text {
                bold:        false,
                color:       Color::new(0x00, 0xFF, 0xAA),
                font_family: "arial".to_string(),
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
            obj_type:   "npc".to_string(),
            visible:    true,
            width:      7777.7,
            x:          6.7,
            y:          7.6,
        }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_text_object() {
        let expecteds: Vec<String> = vec![json! {
            {
                "height":     1111.8,
                "id":         42,
                "name":       "Magica",
                "properties": null,
                "rotation":   77.77,
                "template":   null,
                "text":       {
                    "bold":       false,
                    "color":      "#FF00FFAA",
                    "fontfamily": "arial",
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
                "type":       "npc",
                "visible":    true,
                "width":      7777.7,
                "x":          6.7,
                "y":          7.6
            }
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![TextObject {
            height:     1111.8,
            id:         42,
            name:       "Magica".to_string(),
            properties: None,
            rotation:   77.77,
            template:   None,
            text:       Text {
                bold:        false,
                color:       Color::new(0x00, 0xFF, 0xAA),
                font_family: "arial".to_string(),
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
            obj_type:   "npc".to_string(),
            visible:    true,
            width:      7777.7,
            x:          6.7,
            y:          7.6,
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
