use serde::Deserialize;
use serde::Serialize;

use super::layer::DrawOrder;
use super::object::Object;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct ObjectGroupLayer {
    #[serde(rename = "draworder")]
    pub draw_order: DrawOrder,
    pub id:         i64,
    pub name:       String,
    pub objects:    Vec<Object>,
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

    use crate::tme::models::point_object::PointObject;
    use crate::tme::models::rectangle_object::RectangleObject;
    use serde_json::json;

    #[test]
    fn deserialize_tile_layer() {
        let expecteds = vec![
            ObjectGroupLayer {
                draw_order: DrawOrder::TopDown,
                id:         42,
                name:       "somebody".to_string(),
                objects:    vec![],
                offset_x:   None,
                offset_y:   None,
                opacity:    1.0,
                properties: None,
                start_x:    None,
                start_y:    None,
                visible:    false,
                x:          13,
                y:          37,
            },
            ObjectGroupLayer {
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
            },
        ];

        let actuals: Vec<ObjectGroupLayer> = serde_json::from_value(json! {
            [
                {
                    "draworder":  "topdown",
                    "id":         42,
                    "name":       "somebody",
                    "objects":    [],
                    "offsetx":    null,
                    "offsety":    null,
                    "opacity":    1.0,
                    "properties": null,
                    "startx":     null,
                    "starty":     null,
                    "visible":    false,
                    "x":          13,
                    "y":          37,
                },
                {
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
                            "y":          7.8,
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
                            "y":          7.8,
                        },
                    ],
                    "offsetx":    13.37,
                    "offsety":    42.42,
                    "opacity":    1.0,
                    "properties": null,
                    "startx":     777,
                    "starty":     666,
                    "visible":    true,
                    "x":          13,
                    "y":          37,
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
                    "draworder":  "topdown",
                    "id":         42,
                    "name":       "somebody",
                    "objects":    [],
                    "offsetx":    null,
                    "offsety":    null,
                    "opacity":    1.0,
                    "properties": null,
                    "startx":     null,
                    "starty":     null,
                    "visible":    false,
                    "x":          13,
                    "y":          37,
                }
            },
            json! {
                {
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
                        "y":          7.8,
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
                        "y":          7.8,
                    },
                    ],
                    "offsetx":    13.37,
                    "offsety":    42.42,
                    "opacity":    1.0,
                    "properties": null,
                    "startx":     777,
                    "starty":     666,
                    "visible":    true,
                    "x":          13,
                    "y":          37,
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            ObjectGroupLayer {
                draw_order: DrawOrder::TopDown,
                id:         42,
                name:       "somebody".to_string(),
                objects:    vec![],
                offset_x:   None,
                offset_y:   None,
                opacity:    1.0,
                properties: None,
                start_x:    None,
                start_y:    None,
                visible:    false,
                x:          13,
                y:          37,
            },
            ObjectGroupLayer {
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
