use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct WangColor {
    #[serde(with = "color_serde")]
    pub color:       Color,
    pub name:        String,
    pub probability: f64,
    pub tile:        i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_wang_color() {
        let actuals: Vec<WangColor> = serde_json::from_value(json! {
            [
                {
                    "color": "#FF00FFAA",
                    "name": "somebody",
                    "probability": 42.42,
                    "tile": 666
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<WangColor> = vec![WangColor {
            color:       Color::new(0x00, 0xFF, 0xAA),
            name:        "somebody".to_string(),
            probability: 42.42,
            tile:        666,
        }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_wang_color() {
        let expecteds: Vec<String> = vec![json! {
            {
                "color": "#FF00FFAA",
                "name": "somebody",
                "probability": 42.42,
                "tile": 666
            }
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![WangColor {
            color:       Color::new(0x00, 0xFF, 0xAA),
            name:        "somebody".to_string(),
            probability: 42.42,
            tile:        666,
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
