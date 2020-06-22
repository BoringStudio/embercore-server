use serde::Deserialize;
use serde::Serialize;

use super::object::Object;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct ObjectTemplate {
    #[serde(rename = "type")]
    pub obj_temp_type: ObjectTemplateType,
    pub tileset:       Option<ExternalTileset>,
    pub object:        Object,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ObjectTemplateType {
    Template,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct ExternalTileset {
    pub first_gid: i64,
    pub source:    String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::general_object::GeneralObject;
    use crate::tme::models::point_object::PointObject;
    use serde_json::json;

    #[test]
    fn deserialize_object_template() {
        let actuals: Vec<ObjectTemplate> = serde_json::from_value(json! {
            [
                {
                    "type": "template",
                    "tileset": null,
                    "object": {
                        "point":      true,
                        "height":     7.7,
                        "id":         666,
                        "name":       "point",
                        "properties": null,
                        "rotation":   0.0,
                        "template":   null,
                        "type":       "type",
                        "visible":    false,
                        "width":      13.37,
                        "x":          8.8,
                        "y":          9.9
                    }
                },
                {
                    "type": "template",
                    "tileset": {
                        "first_gid": 777,
                        "source":    "/dev/null"
                    },
                    "object": {
                        "gid":        777,
                        "height":     7.7,
                        "id":         666,
                        "name":       "point",
                        "properties": null,
                        "rotation":   0.0,
                        "template":   null,
                        "type":       "type",
                        "visible":    false,
                        "width":      13.37,
                        "x":          8.8,
                        "y":          9.9
                    }
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<ObjectTemplate> = vec![
            ObjectTemplate {
                obj_temp_type: ObjectTemplateType::Template,
                tileset:       None,
                object:        Object::Point(PointObject {
                    point:      true,
                    height:     7.7,
                    id:         666,
                    name:       "point".to_string(),
                    properties: None,
                    rotation:   0.0,
                    template:   None,
                    obj_type:   "type".to_string(),
                    visible:    false,
                    width:      13.37,
                    x:          8.8,
                    y:          9.9,
                }),
            },
            ObjectTemplate {
                obj_temp_type: ObjectTemplateType::Template,
                tileset:       Some(ExternalTileset {
                    first_gid: 777,
                    source:    "/dev/null".to_string(),
                }),
                object:        Object::General(GeneralObject {
                    gid:        777,
                    height:     7.7,
                    id:         666,
                    name:       "point".to_string(),
                    properties: None,
                    rotation:   0.0,
                    template:   None,
                    obj_type:   "type".to_string(),
                    visible:    false,
                    width:      13.37,
                    x:          8.8,
                    y:          9.9,
                }),
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_object_template() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "type": "template",
                    "tileset": null,
                    "object": {
                        "point":      true,
                        "height":     7.7,
                        "id":         666,
                        "name":       "point",
                        "properties": null,
                        "rotation":   0.0,
                        "template":   null,
                        "type":       "type",
                        "visible":    false,
                        "width":      13.37,
                        "x":          8.8,
                        "y":          9.9
                    }
                }
            },
            json! {
                {
                    "type": "template",
                    "tileset": {
                        "first_gid": 777,
                        "source":    "/dev/null"
                    },
                    "object": {
                        "gid":        777,
                        "height":     7.7,
                        "id":         666,
                        "name":       "point",
                        "properties": null,
                        "rotation":   0.0,
                        "template":   null,
                        "type":       "type",
                        "visible":    false,
                        "width":      13.37,
                        "x":          8.8,
                        "y":          9.9
                    }
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            ObjectTemplate {
                obj_temp_type: ObjectTemplateType::Template,
                tileset:       None,
                object:        Object::Point(PointObject {
                    point:      true,
                    height:     7.7,
                    id:         666,
                    name:       "point".to_string(),
                    properties: None,
                    rotation:   0.0,
                    template:   None,
                    obj_type:   "type".to_string(),
                    visible:    false,
                    width:      13.37,
                    x:          8.8,
                    y:          9.9,
                }),
            },
            ObjectTemplate {
                obj_temp_type: ObjectTemplateType::Template,
                tileset:       Some(ExternalTileset {
                    first_gid: 777,
                    source:    "/dev/null".to_string(),
                }),
                object:        Object::General(GeneralObject {
                    gid:        777,
                    height:     7.7,
                    id:         666,
                    name:       "point".to_string(),
                    properties: None,
                    rotation:   0.0,
                    template:   None,
                    obj_type:   "type".to_string(),
                    visible:    false,
                    width:      13.37,
                    x:          8.8,
                    y:          9.9,
                }),
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
