use serde::Deserialize;
use serde::Serialize;

use super::layer::Layer;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct GroupLayer {
    pub id:         i64,
    pub layers:     Vec<Layer>,
    pub name:       String,
    #[serde(rename = "offsetx")]
    pub offset_x:   Option<f64>,
    #[serde(rename = "offsety")]
    pub offset_y:   Option<f64>,
    pub opacity:    f64,
    pub properties: Option<Vec<Property>>,
    #[serde(rename = "startx")]
    pub start_x:    Option<i64>,
    #[serde(rename = "starty")]
    pub start_y:    Option<i64>,
    pub visible:    bool,
    pub x:          i64,
    pub y:          i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::chunk::Chunk;
    use crate::tme::models::data_source::DataSource::{Encoded, Raw};
    use crate::tme::models::layer::Encoding;
    use crate::tme::models::layer::Layer;
    use crate::tme::models::tile_layer::TileLayer;
    use serde_json::json;

    #[test]
    fn deserialize_tile_layer() {
        let expecteds = vec![
            GroupLayer {
                id:         777,
                layers:     vec![],
                name:       "somebody".to_string(),
                offset_x:   None,
                offset_y:   None,
                opacity:    42.42,
                properties: None,
                start_x:    None,
                start_y:    None,
                visible:    false,
                x:          6,
                y:          6,
            },
            GroupLayer {
                id:         777,
                layers:     vec![
                    Layer::TileLayer(TileLayer {
                        chunks:      None,
                        compression: None,
                        data:        Encoded(
                            "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==".to_owned(),
                        ),
                        encoding:    Some(Encoding::Base64),
                        height:      100,
                        id:          9,
                        name:        "L1".to_owned(),
                        offset_x:    None,
                        offset_y:    None,
                        opacity:     1.0,
                        properties:  None,
                        start_x:     None,
                        start_y:     None,
                        visible:     true,
                        width:       100,
                        x:           0,
                        y:           0,
                    }),
                    Layer::TileLayer(TileLayer {
                        chunks:      Some(vec![
                            Chunk {
                                data:   Raw(vec![1, 1, 0, 1]),
                                height: 15,
                                width:  55,
                                x:      77,
                                y:      88,
                            },
                            Chunk {
                                data:   Encoded("MSwgMSwgMCwgMQ==".to_owned()),
                                height: 6,
                                width:  66,
                                x:      0,
                                y:      0,
                            },
                        ]),
                        compression: None,
                        data:        Encoded(
                            "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==".to_owned(),
                        ),
                        encoding:    Some(Encoding::Base64),
                        height:      100,
                        id:          9,
                        name:        "L1".to_owned(),
                        offset_x:    None,
                        offset_y:    None,
                        opacity:     1.0,
                        properties:  None,
                        start_x:     None,
                        start_y:     None,
                        visible:     true,
                        width:       100,
                        x:           0,
                        y:           0,
                    }),
                ],
                name:       "somebody".to_string(),
                offset_x:   Some(666.666),
                offset_y:   Some(666.666),
                opacity:    42.42,
                properties: None,
                start_x:    Some(1337),
                start_y:    Some(1337),
                visible:    false,
                x:          6,
                y:          6,
            },
        ];

        let actuals: Vec<GroupLayer> = serde_json::from_value(json! {
            [
                {
                    "id":         777,
                    "layers":     [],
                    "name":       "somebody",
                    "offsetx":   null,
                    "offsety":   null,
                    "opacity":    42.42,
                    "properties": null,
                    "startx":    null,
                    "starty":    null,
                    "visible":    false,
                    "x":          6,
                    "y":          6,
                },
                {
                    "id":         777,
                    "layers":     [
                        {
                            "type":        "tilelayer",
                            "chunks":      null,
                            "compression": null,
                            "data":        "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                            "encoding":    "base64",
                            "height":      100,
                            "id":          9,
                            "name":        "L1",
                            "offsetx":    null,
                            "offsety":    null,
                            "opacity":     1.0,
                            "properties":  null,
                            "startx":     null,
                            "starty":     null,
                            "visible":     true,
                            "width":       100,
                            "x":           0,
                            "y":           0,
                        },
                        {
                            "type":        "tilelayer",
                            "chunks":      [
                                {
                                    "data":   [1, 1, 0, 1],
                                    "height": 15,
                                    "width":  55,
                                    "x":      77,
                                    "y":      88,
                                },
                                {
                                    "data":   "MSwgMSwgMCwgMQ==",
                                    "height": 6,
                                    "width":  66,
                                    "x":      0,
                                    "y":      0,
                                },
                            ],
                            "compression": null,
                            "data":        "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                            "encoding":    "base64",
                            "height":      100,
                            "id":          9,
                            "name":        "L1",
                            "offsetx":    null,
                            "offsety":    null,
                            "opacity":     1.0,
                            "properties":  null,
                            "startx":     null,
                            "starty":     null,
                            "visible":     true,
                            "width":       100,
                            "x":           0,
                            "y":           0,
                        },
                    ],
                    "name":       "somebody",
                    "offsetx":   666.666,
                    "offsety":   666.666,
                    "opacity":    42.42,
                    "properties": null,
                    "startx":    1337,
                    "starty":    1337,
                    "visible":    false,
                    "x":          6,
                    "y":          6,
                },
            ]
        })
        .unwrap();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_tile_layer() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "id":         777,
                    "layers":     [],
                    "name":       "somebody",
                    "offsetx":   null,
                    "offsety":   null,
                    "opacity":    42.42,
                    "properties": null,
                    "startx":    null,
                    "starty":    null,
                    "visible":    false,
                    "x":          6,
                    "y":          6,
                }
            },
            json! {
                {
                    "id":         777,
                    "layers":     [
                        {
                            "type":        "tilelayer",
                            "chunks":      null,
                            "compression": null,
                            "data":        "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                            "encoding":    "base64",
                            "height":      100,
                            "id":          9,
                            "name":        "L1",
                            "offsetx":    null,
                            "offsety":    null,
                            "opacity":     1.0,
                            "properties":  null,
                            "startx":     null,
                            "starty":     null,
                            "visible":     true,
                            "width":       100,
                            "x":           0,
                            "y":           0,
                        },
                        {
                            "type":        "tilelayer",
                            "chunks":      [
                                {
                                    "data":   [1, 1, 0, 1],
                                    "height": 15,
                                    "width":  55,
                                    "x":      77,
                                    "y":      88,
                                },
                                {
                                    "data":   "MSwgMSwgMCwgMQ==",
                                    "height": 6,
                                    "width":  66,
                                    "x":      0,
                                    "y":      0,
                                },
                            ],
                            "compression": null,
                            "data":        "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                            "encoding":    "base64",
                            "height":      100,
                            "id":          9,
                            "name":        "L1",
                            "offsetx":    null,
                            "offsety":    null,
                            "opacity":     1.0,
                            "properties":  null,
                            "startx":     null,
                            "starty":     null,
                            "visible":     true,
                            "width":       100,
                            "x":           0,
                            "y":           0,
                        },
                    ],
                    "name":       "somebody",
                    "offsetx":   666.666,
                    "offsety":   666.666,
                    "opacity":    42.42,
                    "properties": null,
                    "startx":    1337,
                    "starty":    1337,
                    "visible":    false,
                    "x":          6,
                    "y":          6,
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            GroupLayer {
                id:         777,
                layers:     vec![],
                name:       "somebody".to_string(),
                offset_x:   None,
                offset_y:   None,
                opacity:    42.42,
                properties: None,
                start_x:    None,
                start_y:    None,
                visible:    false,
                x:          6,
                y:          6,
            },
            GroupLayer {
                id:         777,
                layers:     vec![
                    Layer::TileLayer(TileLayer {
                        chunks:      None,
                        compression: None,
                        data:        Encoded(
                            "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==".to_owned(),
                        ),
                        encoding:    Some(Encoding::Base64),
                        height:      100,
                        id:          9,
                        name:        "L1".to_owned(),
                        offset_x:    None,
                        offset_y:    None,
                        opacity:     1.0,
                        properties:  None,
                        start_x:     None,
                        start_y:     None,
                        visible:     true,
                        width:       100,
                        x:           0,
                        y:           0,
                    }),
                    Layer::TileLayer(TileLayer {
                        chunks:      Some(vec![
                            Chunk {
                                data:   Raw(vec![1, 1, 0, 1]),
                                height: 15,
                                width:  55,
                                x:      77,
                                y:      88,
                            },
                            Chunk {
                                data:   Encoded("MSwgMSwgMCwgMQ==".to_owned()),
                                height: 6,
                                width:  66,
                                x:      0,
                                y:      0,
                            },
                        ]),
                        compression: None,
                        data:        Encoded(
                            "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==".to_owned(),
                        ),
                        encoding:    Some(Encoding::Base64),
                        height:      100,
                        id:          9,
                        name:        "L1".to_owned(),
                        offset_x:    None,
                        offset_y:    None,
                        opacity:     1.0,
                        properties:  None,
                        start_x:     None,
                        start_y:     None,
                        visible:     true,
                        width:       100,
                        x:           0,
                        y:           0,
                    }),
                ],
                name:       "somebody".to_string(),
                offset_x:   Some(666.666),
                offset_y:   Some(666.666),
                opacity:    42.42,
                properties: None,
                start_x:    Some(1337),
                start_y:    Some(1337),
                visible:    false,
                x:          6,
                y:          6,
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
