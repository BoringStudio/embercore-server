use serde::Deserialize;
use serde::Serialize;

use super::layer::Layer;
use super::map::MapType;
use super::map::RenderOrder;
use super::property::Property;
use super::tileset::TilesetContainer;
use super::utils;

use crate::tme::color::opt_color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct OrthogonalMap {
    #[serde(with = "opt_color_serde")]
    #[serde(rename = "backgroundcolor")]
    pub background_color:  Option<Color>,
    #[serde(rename = "compressionlevel")]
    pub compression_level: i64,
    pub height:            i64,
    pub infinite:          bool,
    pub layers:            Vec<Layer>,
    #[serde(rename = "nextlayerid")]
    pub next_layer_id:     i64,
    #[serde(rename = "nextobjectid")]
    pub next_object_id:    i64,
    pub properties:        Option<Vec<Property>>,
    #[serde(rename = "renderorder")]
    pub render_order:      RenderOrder,
    #[serde(rename = "tiledversion")]
    pub tiled_version:     String,
    #[serde(rename = "tileheight")]
    pub tile_height:       i64,
    #[serde(rename = "tilesets")]
    pub tile_sets:         Vec<TilesetContainer>,
    #[serde(rename = "tilewidth")]
    pub tile_width:        i64,
    #[serde(rename = "type")]
    pub map_type:          MapType,
    #[serde(deserialize_with = "utils::deserialize_value_to_string")]
    pub version:           String,
    pub width:             i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::chunk::Chunk;
    use crate::tme::models::data_source::DataSource::{Encoded, Raw};
    use crate::tme::models::layer::{DrawOrder, Encoding};
    use crate::tme::models::object::Object;
    use crate::tme::models::object_group_layer::ObjectGroupLayer;
    use crate::tme::models::point_object::PointObject;
    use crate::tme::models::rectangle_object::RectangleObject;
    use crate::tme::models::tile_layer::TileLayer;
    use serde_json::json;

    #[test]
    fn deserialize_isometric_map() {
        let actuals: Vec<OrthogonalMap> = serde_json::from_value(json! {
            [
                {
                    "backgroundcolor":  null,
                    "compressionlevel": -1,
                    "height":           77,
                    "infinite":         true,
                    "layers":           [],
                    "nextlayerid":      0,
                    "nextobjectid":     0,
                    "properties":       null,
                    "renderorder":      "right-down",
                    "tiledversion":     "1.3.5",
                    "tileheight":       99,
                    "tilesets":         [],
                    "tilewidth":        89,
                    "type":             "map",
                    "version":          1.2,
                    "width":            77
                },
                {
                    "backgroundcolor":  null,
                    "compressionlevel": -1,
                    "height":           77,
                    "infinite":         true,
                    "layers":           [
                        {
                            "type":   "tilelayer",
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
                        },
                        {
                            "type":       "objectgroup",
                            "draworder":  "index",
                            "id":         42,
                            "name":       "somebody",
                            "objects":    [
                                {
                                    "point":      true,
                                    "height":     11.11,
                                    "id":         255,
                                    "name":       "B-baka",
                                    "properties": null,
                                    "rotation":   777.7,
                                    "template":   "template",
                                    "type":       "npc",
                                    "visible":    false,
                                    "width":      9.4,
                                    "x":          5.6,
                                    "y":          7.8
                                },
                                {
                                    "height":     11.11,
                                    "id":         255,
                                    "name":       "Senpai",
                                    "properties": null,
                                    "rotation":   0.0,
                                    "template":   null,
                                    "type":       "npc",
                                    "visible":    false,
                                    "width":      9.4,
                                    "x":          5.6,
                                    "y":          7.8
                                }
                            ],
                            "offsetx":    13.37,
                            "offsety":    42.42,
                            "opacity":    1.0,
                            "properties": null,
                            "startx":     777,
                            "starty":     666,
                            "visible":    true,
                            "x":          13,
                            "y":          37
                        }
                    ],
                    "nextlayerid":  0,
                    "nextobjectid": 0,
                    "properties":   null,
                    "renderorder":  "left-up",
                    "tiledversion": "1.3.5",
                    "tileheight":   99,
                    "tilesets":     [],
                    "tilewidth":    89,
                    "type":         "map",
                    "version":      1.2,
                    "width":        77
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<OrthogonalMap> = vec![
            OrthogonalMap {
                background_color:  None,
                compression_level: -1,
                height:            77,
                infinite:          true,
                layers:            vec![],
                next_layer_id:     0,
                next_object_id:    0,
                properties:        None,
                render_order:      RenderOrder::RightDown,
                tiled_version:     "1.3.5".to_string(),
                tile_height:       99,
                tile_sets:         vec![],
                tile_width:        89,
                map_type:          MapType::Map,
                version:           "1.2".to_string(),
                width:             77,
            },
            OrthogonalMap {
                background_color:  None,
                compression_level: -1,
                height:            77,
                infinite:          true,
                layers:            vec![
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
                    Layer::ObjectGroupLayer(ObjectGroupLayer {
                        draw_order: DrawOrder::Index,
                        id:         42,
                        name:       "somebody".to_string(),
                        objects:    vec![
                            Object::Point(PointObject {
                                point:      true,
                                height:     11.11,
                                id:         255,
                                name:       "B-baka".to_string(),
                                properties: None,
                                rotation:   777.7,
                                template:   Some("template".to_string()),
                                obj_type:   "npc".to_string(),
                                visible:    false,
                                width:      9.4,
                                x:          5.6,
                                y:          7.8,
                            }),
                            Object::Rectangle(RectangleObject {
                                height:     11.11,
                                id:         255,
                                name:       "Senpai".to_string(),
                                properties: None,
                                rotation:   0.0,
                                template:   None,
                                obj_type:   "npc".to_string(),
                                visible:    false,
                                width:      9.4,
                                x:          5.6,
                                y:          7.8,
                            }),
                        ],
                        offset_x:   Some(13.37),
                        offset_y:   Some(42.42),
                        opacity:    1.0,
                        properties: None,
                        start_x:    Some(777),
                        start_y:    Some(666),
                        visible:    true,
                        x:          13,
                        y:          37,
                    }),
                ],
                next_layer_id:     0,
                next_object_id:    0,
                properties:        None,
                render_order:      RenderOrder::LeftUp,
                tiled_version:     "1.3.5".to_string(),
                tile_height:       99,
                tile_sets:         vec![],
                tile_width:        89,
                map_type:          MapType::Map,
                version:           "1.2".to_string(),
                width:             77,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_isometric_map() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "backgroundcolor":  null,
                    "compressionlevel": -1,
                    "height":           77,
                    "infinite":         true,
                    "layers":           [],
                    "nextlayerid":      0,
                    "nextobjectid":     0,
                    "properties":       null,
                    "renderorder":      "right-down",
                    "tiledversion":     "1.3.5",
                    "tileheight":       99,
                    "tilesets":         [],
                    "tilewidth":        89,
                    "type":             "map",
                    "version":          "1.2",
                    "width":            77,
                }
            },
            json! {
                {
                    "backgroundcolor":  null,
                    "compressionlevel": -1,
                    "height":           77,
                    "infinite":         true,
                    "layers":           [
                        {
                            "type":   "tilelayer",
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
                        },
                        {
                            "type":       "objectgroup",
                            "draworder":  "index",
                            "id":         42,
                            "name":       "somebody",
                            "objects":    [
                                {
                                    "point":      true,
                                    "height":     11.11,
                                    "id":         255,
                                    "name":       "B-baka",
                                    "properties": null,
                                    "rotation":   777.7,
                                    "template":   "template",
                                    "type":       "npc",
                                    "visible":    false,
                                    "width":      9.4,
                                    "x":          5.6,
                                    "y":          7.8
                                },
                                {
                                    "height":     11.11,
                                    "id":         255,
                                    "name":       "Senpai",
                                    "properties": null,
                                    "rotation":   0.0,
                                    "template":   null,
                                    "type":       "npc",
                                    "visible":    false,
                                    "width":      9.4,
                                    "x":          5.6,
                                    "y":          7.8
                                }
                            ],
                            "offsetx":    13.37,
                            "offsety":    42.42,
                            "opacity":    1.0,
                            "properties": null,
                            "startx":     777,
                            "starty":     666,
                            "visible":    true,
                            "x":          13,
                            "y":          37
                        }
                    ],
                    "nextlayerid":  0,
                    "nextobjectid": 0,
                    "properties":   null,
                    "renderorder":  "left-up",
                    "tiledversion": "1.3.5",
                    "tileheight":   99,
                    "tilesets":     [],
                    "tilewidth":    89,
                    "type":         "map",
                    "version":      "1.2",
                    "width":        77
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            OrthogonalMap {
                background_color:  None,
                compression_level: -1,
                height:            77,
                infinite:          true,
                layers:            vec![],
                next_layer_id:     0,
                next_object_id:    0,
                properties:        None,
                render_order:      RenderOrder::RightDown,
                tiled_version:     "1.3.5".to_string(),
                tile_height:       99,
                tile_sets:         vec![],
                tile_width:        89,
                map_type:          MapType::Map,
                version:           "1.2".to_string(),
                width:             77,
            },
            OrthogonalMap {
                background_color:  None,
                compression_level: -1,
                height:            77,
                infinite:          true,
                layers:            vec![
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
                    Layer::ObjectGroupLayer(ObjectGroupLayer {
                        draw_order: DrawOrder::Index,
                        id:         42,
                        name:       "somebody".to_string(),
                        objects:    vec![
                            Object::Point(PointObject {
                                point:      true,
                                height:     11.11,
                                id:         255,
                                name:       "B-baka".to_string(),
                                properties: None,
                                rotation:   777.7,
                                template:   Some("template".to_string()),
                                obj_type:   "npc".to_string(),
                                visible:    false,
                                width:      9.4,
                                x:          5.6,
                                y:          7.8,
                            }),
                            Object::Rectangle(RectangleObject {
                                height:     11.11,
                                id:         255,
                                name:       "Senpai".to_string(),
                                properties: None,
                                rotation:   0.0,
                                template:   None,
                                obj_type:   "npc".to_string(),
                                visible:    false,
                                width:      9.4,
                                x:          5.6,
                                y:          7.8,
                            }),
                        ],
                        offset_x:   Some(13.37),
                        offset_y:   Some(42.42),
                        opacity:    1.0,
                        properties: None,
                        start_x:    Some(777),
                        start_y:    Some(666),
                        visible:    true,
                        x:          13,
                        y:          37,
                    }),
                ],
                next_layer_id:     0,
                next_object_id:    0,
                properties:        None,
                render_order:      RenderOrder::LeftUp,
                tiled_version:     "1.3.5".to_string(),
                tile_height:       99,
                tile_sets:         vec![],
                tile_width:        89,
                map_type:          MapType::Map,
                version:           "1.2".to_string(),
                width:             77,
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
