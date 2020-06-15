use serde::Deserialize;
use serde::Serialize;

use super::chunk::Chunk;
use super::data_source::DataSource;
use super::layer::Compression;
use super::layer::Encoding;
use super::property::Property;
use super::utils;

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
    use crate::tme::models::layer::Layer;

    const LAYERS_STR: &'static str = r#"
            [
              {
                "data":[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                "height":100,
                "id":9,
                "name":"L1",
                "opacity":1,
                "type":"tilelayer",
                "visible":true,
                "width":100,
                "x":0,
                "y":0
              },
              {
                "data": "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                "encoding": "base64",
                "height":100,
                "id":9,
                "name":"L1",
                "opacity":1,
                "type":"tilelayer",
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
                "data": "MCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMCwgMA==",
                "encoding": "base64",
                "height":100,
                "id":9,
                "name":"L1",
                "opacity":1,
                "type":"tilelayer",
                "visible":true,
                "width":100,
                "x":0,
                "y":0
              },
              {
                "compression": "zlib",
                "data": "eNrzdS5P9yUFO9raAgDo2RIx",
                "encoding": "base64",
                "height":100,
                "id":9,
                "name":"L1",
                "opacity":1,
                "type":"tilelayer",
                "visible":true,
                "width":100,
                "x":0,
                "y":0
              },
              {
                "compression": "gzip",
                "data": "H4sIAAAAAAAA//N1Lk/3JQU72toCAHCxT1Y0AAAA",
                "encoding": "base64",
                "height":100,
                "id":9,
                "name":"L1",
                "opacity":1,
                "type":"tilelayer",
                "visible":true,
                "width":100,
                "x":0,
                "y":0
              }
            ]
        "#;

    #[test]
    fn deserialize_tile_layer() {
        let expected_layers = vec![
            Layer::TileLayer(TileLayer {
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
            }),
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
            Layer::TileLayer(TileLayer {
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
            }),
            Layer::TileLayer(TileLayer {
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
            }),
        ];

        let actual_layers: Vec<Layer> = serde_json::from_str(LAYERS_STR).unwrap();

        for (actual, expected) in actual_layers.into_iter().zip(expected_layers) {
            match actual {
                Layer::TileLayer(a) => {
                    if let Layer::TileLayer(e) = expected {
                        assert_eq!(a, e);
                    } else {
                        panic!("invalid expected layer type")
                    }
                }
                _ => panic!("invalid actual layer type"),
            }
        }
    }
}
