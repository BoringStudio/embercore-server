use serde::Deserialize;
use serde::Serialize;

use super::point::Point;
use super::property::Property;
use super::utils;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct PolygonObject {
    pub height:     f64,
    pub id:         i64,
    pub name:       String,
    #[serde(default = "utils::make_none_option")]
    pub properties: Option<Vec<Property>>,
    pub polygon:    Vec<Point>,
    pub rotation:   f64,
    #[serde(default = "utils::make_none_option")]
    pub template:   Option<String>,
    #[serde(rename = "type")]
    pub obj_type:   String,
    pub visible:    bool,
    pub width:      f64,
    pub x:          f64,
    pub y:          f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_polygon_object() {
        let actuals: Vec<PolygonObject> = serde_json::from_value(json! {
            [
                {
                    "height":     1111.8,
                    "id":         42,
                    "name":       "Madoka",
                    "properties": null,
                    "polygon":    [],
                    "rotation":   77.77,
                    "template":   null,
                    "type":       "npc",
                    "visible":    true,
                    "width":      7777.7,
                    "x":          6.7,
                    "y":          7.6
                },
                {
                    "height":     1111.8,
                    "id":         42,
                    "name":       "Magica",
                    "properties": null,
                    "polygon":    [
                        {
                            "x": 77.7,
                            "y": 66.6
                        },
                        {
                            "x": 13.37,
                            "y": 7777.6
                        }
                    ],
                    "rotation":   77.77,
                    "template":   "null",
                    "type":       "npc",
                    "visible":    true,
                    "width":      7777.7,
                    "x":          6.7,
                    "y":          7.6
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<PolygonObject> = vec![
            PolygonObject {
                height:     1111.8,
                id:         42,
                name:       "Madoka".to_string(),
                properties: None,
                polygon:    vec![],
                rotation:   77.77,
                template:   None,
                obj_type:   "npc".to_string(),
                visible:    true,
                width:      7777.7,
                x:          6.7,
                y:          7.6,
            },
            PolygonObject {
                height:     1111.8,
                id:         42,
                name:       "Magica".to_string(),
                properties: None,
                polygon:    vec![
                    Point { x: 77.7, y: 66.6 },
                    Point {
                        x: 13.37,
                        y: 7777.6,
                    },
                ],
                rotation:   77.77,
                template:   Some("null".to_string()),
                obj_type:   "npc".to_string(),
                visible:    true,
                width:      7777.7,
                x:          6.7,
                y:          7.6,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_polygon_object() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "height":     1111.8,
                    "id":         42,
                    "name":       "Madoka",
                    "properties": null,
                    "polygon":    [],
                    "rotation":   77.77,
                    "template":   null,
                    "type":       "npc",
                    "visible":    true,
                    "width":      7777.7,
                    "x":          6.7,
                    "y":          7.6
                }
            },
            json! {
                {
                    "height":     1111.8,
                    "id":         42,
                    "name":       "Magica",
                    "properties": null,
                    "polygon":    [
                        {
                            "x": 77.7,
                            "y": 66.6
                        },
                        {
                            "x": 13.37,
                            "y": 7777.6
                        }
                    ],
                    "rotation":   77.77,
                    "template":   "null",
                    "type":       "npc",
                    "visible":    true,
                    "width":      7777.7,
                    "x":          6.7,
                    "y":          7.6
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            PolygonObject {
                height:     1111.8,
                id:         42,
                name:       "Madoka".to_string(),
                properties: None,
                polygon:    vec![],
                rotation:   77.77,
                template:   None,
                obj_type:   "npc".to_string(),
                visible:    true,
                width:      7777.7,
                x:          6.7,
                y:          7.6,
            },
            PolygonObject {
                height:     1111.8,
                id:         42,
                name:       "Magica".to_string(),
                properties: None,
                polygon:    vec![
                    Point { x: 77.7, y: 66.6 },
                    Point {
                        x: 13.37,
                        y: 7777.6,
                    },
                ],
                rotation:   77.77,
                template:   Some("null".to_string()),
                obj_type:   "npc".to_string(),
                visible:    true,
                width:      7777.7,
                x:          6.7,
                y:          7.6,
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
