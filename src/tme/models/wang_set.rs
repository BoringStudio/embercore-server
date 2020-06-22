use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use super::wang_color::WangColor;
use super::wang_tile::WangTile;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct WangSet {
    #[serde(rename = "cornercolors")]
    pub corner_colors: Vec<WangColor>,
    #[serde(rename = "edgecolors")]
    pub edge_colors:   Vec<WangColor>,
    pub name:          String,
    pub properties:    Option<Vec<Property>>,
    pub tile:          i64,
    #[serde(rename = "wangtiles")]
    pub wang_tiles:    Vec<WangTile>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::color::Color;
    use serde_json::json;

    #[test]
    fn deserialize_wang_set() {
        let actuals: Vec<WangSet> = serde_json::from_value(json! {
            [
                {
                    "cornercolors": [
                        {
                            "color":       "#FF00FFAA",
                            "name":        "color",
                            "probability": 1.0,
                            "tile":        999,
                        },
                        {
                            "color":       "#FF774400",
                            "name":        "color",
                            "probability": 1.0,
                            "tile":        8,
                        },
                    ],
                    "edgecolors": [
                        {
                            "color":       "#FF00FFAA",
                            "name":        "color",
                            "probability": 1.0,
                            "tile":        999,
                        },
                        {
                            "color":       "#FF774400",
                            "name":        "color",
                            "probability": 1.0,
                            "tile":        8,
                        },
                    ],
                    "name":       "Madoka",
                    "properties": null,
                    "tile":       7777,
                    "wangtiles":  [
                        {
                            "dflip":  true,
                            "hflip":  true,
                            "tileid": 11111,
                            "vflip":  true,
                            "wangid": [8, 7]
                        }
                    ]
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<WangSet> = vec![WangSet {
            corner_colors: vec![
                WangColor {
                    color:       Color::new(0x00, 0xFF, 0xAA),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        999,
                },
                WangColor {
                    color:       Color::new(0x77, 0x44, 0x00),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        8,
                },
            ],
            edge_colors:   vec![
                WangColor {
                    color:       Color::new(0x00, 0xFF, 0xAA),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        999,
                },
                WangColor {
                    color:       Color::new(0x77, 0x44, 0x00),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        8,
                },
            ],
            name:          "Madoka".to_string(),
            properties:    None,
            tile:          7777,
            wang_tiles:    vec![WangTile {
                d_flip:  true,
                h_flip:  true,
                tile_id: 11111,
                v_flip:  true,
                wang_id: vec![8, 7],
            }],
        }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_wang_set() {
        let expecteds: Vec<String> = vec![json! {
            {
                "cornercolors": [
                    {
                        "color":       "#FF00FFAA",
                        "name":        "color",
                        "probability": 1.0,
                        "tile":        999,
                    },
                    {
                        "color":       "#FF774400",
                        "name":        "color",
                        "probability": 1.0,
                        "tile":        8,
                    },
                ],
                "edgecolors": [
                    {
                        "color":       "#FF00FFAA",
                        "name":        "color",
                        "probability": 1.0,
                        "tile":        999,
                    },
                    {
                        "color":       "#FF774400",
                        "name":        "color",
                        "probability": 1.0,
                        "tile":        8,
                    },
                ],
                "name":       "Madoka",
                "properties": null,
                "tile":       7777,
                "wangtiles":  [
                    {
                        "dflip":  true,
                        "hflip":  true,
                        "tileid": 11111,
                        "vflip":  true,
                        "wangid": [8, 7]
                    }
                ]
            }
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![WangSet {
            corner_colors: vec![
                WangColor {
                    color:       Color::new(0x00, 0xFF, 0xAA),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        999,
                },
                WangColor {
                    color:       Color::new(0x77, 0x44, 0x00),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        8,
                },
            ],
            edge_colors:   vec![
                WangColor {
                    color:       Color::new(0x00, 0xFF, 0xAA),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        999,
                },
                WangColor {
                    color:       Color::new(0x77, 0x44, 0x00),
                    name:        "color".to_string(),
                    probability: 1.0,
                    tile:        8,
                },
            ],
            name:          "Madoka".to_string(),
            properties:    None,
            tile:          7777,
            wang_tiles:    vec![WangTile {
                d_flip:  true,
                h_flip:  true,
                tile_id: 11111,
                v_flip:  true,
                wang_id: vec![8, 7],
            }],
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
