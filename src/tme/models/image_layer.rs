use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use crate::tme::color::opt_color_serde;
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
    #[serde(with = "opt_color_serde")]
    #[serde(rename = "transparentcolor")]
    pub transparent_color: Option<Color>,
    pub visible:           bool,
    pub x:                 i64,
    pub y:                 i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_image_layer() {
        let expecteds = vec![
            ImageLayer {
                id:                777,
                image:             "/dev/null".to_string(),
                name:              "somebody".to_string(),
                offset_x:          None,
                offset_y:          None,
                opacity:           42.42,
                properties:        None,
                start_x:           None,
                start_y:           None,
                transparent_color: None,
                visible:           true,
                x:                 777,
                y:                 666,
            },
            ImageLayer {
                id:                123,
                image:             "/dev/null".to_string(),
                name:              "somebody".to_string(),
                offset_x:          Some(0.1),
                offset_y:          Some(2.3),
                opacity:           42.42,
                properties:        None,
                start_x:           Some(4),
                start_y:           Some(5),
                transparent_color: Some(Color::new(0x00, 0xFF, 0xAA)),
                visible:           true,
                x:                 777,
                y:                 666,
            },
        ];

        let actuals: Vec<ImageLayer> = serde_json::from_value(json! {
            [
                {
                    "id":               777,
                    "image":            "/dev/null".to_string(),
                    "name":             "somebody".to_string(),
                    "offsetx":          null,
                    "offsety":          null,
                    "opacity":          42.42,
                    "properties":       null,
                    "startx":           null,
                    "starty":           null,
                    "transparentcolor": null,
                    "visible":          true,
                    "x":                777,
                    "y":                666,
                },
                {
                    "id":               123,
                    "image":            "/dev/null",
                    "name":             "somebody",
                    "offsetx":          0.1,
                    "offsety":          2.3,
                    "opacity":          42.42,
                    "properties":       null,
                    "startx":           4,
                    "starty":           5,
                    "transparentcolor": "#FF00FFAA",
                    "visible":          true,
                    "x":                777,
                    "y":                666,
                }
            ]
        })
        .unwrap();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_image_layer() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "id":               777,
                    "image":            "/dev/null".to_string(),
                    "name":             "somebody".to_string(),
                    "offsetx":          null,
                    "offsety":          null,
                    "opacity":          42.42,
                    "properties":       null,
                    "startx":           null,
                    "starty":           null,
                    "transparentcolor": null,
                    "visible":          true,
                    "x":                777,
                    "y":                666,
                }
            },
            json! {
                {
                    "id":               123,
                    "image":            "/dev/null",
                    "name":             "somebody",
                    "offsetx":          0.1,
                    "offsety":          2.3,
                    "opacity":          42.42,
                    "properties":       null,
                    "startx":           4,
                    "starty":           5,
                    "transparentcolor": "#FF00FFAA",
                    "visible":          true,
                    "x":                777,
                    "y":                666,
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            ImageLayer {
                id:                777,
                image:             "/dev/null".to_string(),
                name:              "somebody".to_string(),
                offset_x:          None,
                offset_y:          None,
                opacity:           42.42,
                properties:        None,
                start_x:           None,
                start_y:           None,
                transparent_color: None,
                visible:           true,
                x:                 777,
                y:                 666,
            },
            ImageLayer {
                id:                123,
                image:             "/dev/null".to_string(),
                name:              "somebody".to_string(),
                offset_x:          Some(0.1),
                offset_y:          Some(2.3),
                opacity:           42.42,
                properties:        None,
                start_x:           Some(4),
                start_y:           Some(5),
                transparent_color: Some(Color::new(0x00, 0xFF, 0xAA)),
                visible:           true,
                x:                 777,
                y:                 666,
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
