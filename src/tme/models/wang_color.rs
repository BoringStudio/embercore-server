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

    use lazy_static::*;

    lazy_static! {
        static ref DE_WANG_COLORS_STR: String = r##"
            [
                {
                    "color": "#FF00FFAA",
                    "name": "somebody",
                    "probability": 42.42,
                    "tile": 666
                }
            ]
        "##
        .to_string();
        static ref SER_WANG_COLORS_STR: Vec<String> = vec![r##"
                {
                    "color": "#FF00FFAA",
                    "name": "somebody",
                    "probability": 42.42,
                    "tile": 666
                }
            "##
        .to_string(),]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_wang_color() {
        let actuals: Vec<WangColor> = serde_json::from_str(DE_WANG_COLORS_STR.as_str()).unwrap();

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
        let expecteds: Vec<String> = SER_WANG_COLORS_STR.to_vec();

        let actuals: Vec<String> = vec![serde_json::to_string(&WangColor {
            color:       Color::new(0x00, 0xFF, 0xAA),
            name:        "somebody".to_string(),
            probability: 42.42,
            tile:        666,
        })
        .unwrap()];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
