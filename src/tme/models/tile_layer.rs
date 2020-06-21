use serde::Deserialize;
use serde::Serialize;

use super::chunk::Chunk;
use super::data_source::DataSource;
use super::layer::Compression;
use super::layer::Encoding;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct TileLayer {
    pub chunks:      Option<Vec<Chunk>>,
    pub compression: Option<Compression>,
    pub data:        DataSource,
    pub encoding:    Option<Encoding>,
    pub height:      i64,
    pub id:          i64,
    pub name:        String,
    #[serde(rename = "offsetx")]
    pub offset_x:    Option<f64>,
    #[serde(rename = "offsety")]
    pub offset_y:    Option<f64>,
    pub opacity:     f64,
    pub properties:  Option<Vec<Property>>,
    #[serde(rename = "startx")]
    pub start_x:     Option<i64>,
    #[serde(rename = "starty")]
    pub start_y:     Option<i64>,
    pub visible:     bool,
    pub width:       i64,
    pub x:           i64,
    pub y:           i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::data_source::DataSource::{Encoded, Raw};
    use serde_json::json;

    #[test]
    fn deserialize_tile_layer() {
        let expected_layers = vec![
            TileLayer {
                chunks:      None,
                compression: None,
                data:        Raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                encoding:    None,
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
            },
            TileLayer {
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
            },
            TileLayer {
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
            },
            TileLayer {
                chunks:      None,
                compression: Some(Compression::Zlib),
                data:        Encoded("eNrzdS5P9yUFO9raAgDo2RIx".to_owned()),
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
            },
            TileLayer {
                chunks:      None,
                compression: Some(Compression::Gzip),
                data:        Encoded("H4sIAAAAAAAA//N1Lk/3JQU72toCAHCxT1Y0AAAA".to_owned()),
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
            },
        ];

        let actual_layers: Vec<TileLayer> = serde_json::from_value(json! {
            [
                {
                    "chunks": null,
                    "compression": null,
                    "data":[0,0,0,0,0,0,0,0,0,0,0,0,0],
                    "encoding": null,
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                },
                {
                    "chunks": null,
                    "compression": null,
                    "data": "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                },
                {
                    "chunks": [
                        {
                            "data": [1, 1, 0, 1],
                            "height": 15,
                            "width": 55,
                            "x": 77,
                            "y": 88
                        },
                        {
                            "data": "MSwgMSwgMCwgMQ==",
                            "height": 6,
                            "width": 66,
                            "x": 0,
                            "y": 0
                        }
                    ],
                    "compression": null,
                    "data": "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                },
                {
                    "chunks": null,
                    "compression": "zlib",
                    "data": "eNrzdS5P9yUFO9raAgDo2RIx",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "opacity":1.0,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                },
                {
                    "chunks": null,
                    "compression": "gzip",
                    "data": "H4sIAAAAAAAA//N1Lk/3JQU72toCAHCxT1Y0AAAA",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                }
            ]
        })
        .unwrap();

        for (actual, expected) in actual_layers.into_iter().zip(expected_layers) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_tile_layer() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "chunks": null,
                    "compression": null,
                    "data":[0,0,0,0,0,0,0,0,0,0,0,0,0],
                    "encoding": null,
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "properties": null,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                }
            },
            json! {
                {
                    "chunks": null,
                    "compression": null,
                    "data": "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "properties": null,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                }
            },
            json! {
                {
                    "chunks": [
                    {
                        "data": [1, 1, 0, 1],
                        "height": 15,
                        "width": 55,
                        "x": 77,
                        "y": 88
                    },
                    {
                        "data": "MSwgMSwgMCwgMQ==",
                        "height": 6,
                        "width": 66,
                        "x": 0,
                        "y": 0
                    }
                    ],
                    "compression": null,
                    "data": "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "properties": null,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                }
            },
            json! {
                {
                    "chunks": null,
                    "compression": "zlib",
                    "data": "eNrzdS5P9yUFO9raAgDo2RIx",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "properties": null,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                }
            },
            json! {
                {
                    "chunks": null,
                    "compression": "gzip",
                    "data": "H4sIAAAAAAAA//N1Lk/3JQU72toCAHCxT1Y0AAAA",
                    "encoding": "base64",
                    "height":100,
                    "id":9,
                    "name":"L1",
                    "offsetx": null,
                    "offsety": null,
                    "opacity":1.0,
                    "properties": null,
                    "startx": null,
                    "starty": null,
                    "visible":true,
                    "width":100,
                    "x":0,
                    "y":0
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            TileLayer {
                chunks:      None,
                compression: None,
                data:        Raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                encoding:    None,
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
            },
            TileLayer {
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
            },
            TileLayer {
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
            },
            TileLayer {
                chunks:      None,
                compression: Some(Compression::Zlib),
                data:        Encoded("eNrzdS5P9yUFO9raAgDo2RIx".to_owned()),
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
            },
            TileLayer {
                chunks:      None,
                compression: Some(Compression::Gzip),
                data:        Encoded("H4sIAAAAAAAA//N1Lk/3JQU72toCAHCxT1Y0AAAA".to_owned()),
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
