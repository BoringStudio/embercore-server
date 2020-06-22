use serde::Deserialize;
use serde::Serialize;

use super::frame::Frame;
use super::layer::Layer;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Tile {
    pub animation:    Option<Vec<Frame>>,
    pub id:           i64,
    pub image:        Option<String>,
    #[serde(rename = "imageheight")]
    pub image_height: Option<i64>,
    #[serde(rename = "imagewidth")]
    pub image_width:  Option<i64>,
    #[serde(rename = "objectgroup")]
    pub object_group: Option<Layer>,
    pub probability:  Option<f64>,
    pub properties:   Option<Vec<Property>>,
    pub terrain:      Option<Vec<i64>>,
    #[serde(rename = "type")]
    pub tile_type:    Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::layer::DrawOrder;
    use crate::tme::models::object::Object;
    use crate::tme::models::object_group_layer::ObjectGroupLayer;
    use crate::tme::models::point_object::PointObject;
    use crate::tme::models::rectangle_object::RectangleObject;
    use serde_json::json;

    #[test]
    fn deserialize_tile() {
        let actuals: Vec<Tile> = serde_json::from_value(json! {
            [
                {
                    "animation":   null,
                    "id":          8888,
                    "image":       null,
                    "imageheight": null,
                    "imagewidth":  null,
                    "objectgroup": null,
                    "probability": null,
                    "properties":  null,
                    "terrain":     null,
                    "type":        null
                },
                {
                    "animation":   [
                        {
                            "duration": 0,
                            "tiledid":  0
                        }
                     ],
                    "id":          8888,
                    "image":       "/dev/null",
                    "imageheight": 6666,
                    "imagewidth":  7777,
                    "objectgroup": {
                        "type":      "objectgroup",
                        "draworder": "index",
                        "id":        42,
                        "name":      "somebody",
                        "objects":   [
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
                    },
                    "probability": 42.42,
                    "properties":  null,
                    "terrain":     [7, 8],
                    "type":        "TYPE"
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Tile> = vec![
            Tile {
                animation:    None,
                id:           8888,
                image:        None,
                image_height: None,
                image_width:  None,
                object_group: None,
                probability:  None,
                properties:   None,
                terrain:      None,
                tile_type:    None,
            },
            Tile {
                animation:    Some(vec![Frame {
                    duration: 0,
                    tiled_id: 0,
                }]),
                id:           8888,
                image:        Some("/dev/null".to_string()),
                image_height: Some(6666),
                image_width:  Some(7777),
                object_group: Some(Layer::ObjectGroupLayer(ObjectGroupLayer {
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
                })),
                probability:  Some(42.42),
                properties:   None,
                terrain:      Some(vec![7, 8]),
                tile_type:    Some("TYPE".to_string()),
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_tile() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "animation":   null,
                    "id":          8888,
                    "image":       null,
                    "imageheight": null,
                    "imagewidth":  null,
                    "objectgroup": null,
                    "probability": null,
                    "properties":  null,
                    "terrain":     null,
                    "type":        null
                }

            },
            json! {
                {
                    "animation":   [
                        {
                            "duration": 0,
                            "tiledid":  0
                        }
                     ],
                    "id":          8888,
                    "image":       "/dev/null",
                    "imageheight": 6666,
                    "imagewidth":  7777,
                    "objectgroup": {
                        "type":      "objectgroup",
                        "draworder": "index",
                        "id":        42,
                        "name":      "somebody",
                        "objects":   [
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
                    },
                    "probability": 42.42,
                    "properties":  null,
                    "terrain":     [7, 8],
                    "type":        "TYPE"
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Tile {
                animation:    None,
                id:           8888,
                image:        None,
                image_height: None,
                image_width:  None,
                object_group: None,
                probability:  None,
                properties:   None,
                terrain:      None,
                tile_type:    None,
            },
            Tile {
                animation:    Some(vec![Frame {
                    duration: 0,
                    tiled_id: 0,
                }]),
                id:           8888,
                image:        Some("/dev/null".to_string()),
                image_height: Some(6666),
                image_width:  Some(7777),
                object_group: Some(Layer::ObjectGroupLayer(ObjectGroupLayer {
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
                })),
                probability:  Some(42.42),
                properties:   None,
                terrain:      Some(vec![7, 8]),
                tile_type:    Some("TYPE".to_string()),
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
