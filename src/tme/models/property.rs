use serde::Deserialize;
use serde::Serialize;

use std::path::PathBuf;

use crate::tme::color::color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Property {
    Int(IntProperty),
    Bool(BoolProperty),
    File(FileProperty),
    Color(ColorProperty),
    Float(FloatProperty),
    String(StringProperty),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IntProperty {
    pub name:  String,
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BoolProperty {
    pub name:  String,
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FileProperty {
    pub name:  String,
    pub value: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ColorProperty {
    pub name:  String,
    #[serde(with = "color_serde")]
    pub value: Color,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FloatProperty {
    pub name:  String,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StringProperty {
    pub name:  String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_property() {
        let actuals: Vec<Property> = serde_json::from_value(json! {
            [
                {
                    "type": "int",
                    "name": "somebody",
                    "value": 42
                },
                {
                    "type": "bool",
                    "name": "somebody",
                    "value": true
                },
                {
                    "type": "file",
                    "name": "somebody",
                    "value": "/dev/null"
                },
                {
                    "type": "color",
                    "name": "somebody",
                    "value": "00FFAA"
                },
                {
                    "type": "float",
                    "name": "somebody",
                    "value": 42.42
                },
                {
                    "type": "string",
                    "name": "somebody",
                    "value": "onestoldme"
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Property> = vec![
            Property::Int(IntProperty {
                name:  "somebody".to_owned(),
                value: 42,
            }),
            Property::Bool(BoolProperty {
                name:  "somebody".to_owned(),
                value: true,
            }),
            Property::File(FileProperty {
                name:  "somebody".to_owned(),
                value: PathBuf::from("/dev/null"),
            }),
            Property::Color(ColorProperty {
                name:  "somebody".to_owned(),
                value: Color::new(0x00, 0xFF, 0xAA),
            }),
            Property::Float(FloatProperty {
                name:  "somebody".to_owned(),
                value: 42.42,
            }),
            Property::String(StringProperty {
                name:  "somebody".to_owned(),
                value: "onestoldme".to_owned(),
            }),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_property() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "type": "int",
                    "name": "somebody",
                    "value": 42
                }
            },
            json! {
                {
                    "type": "bool",
                    "name": "somebody",
                    "value": true
                }
            },
            json! {
                {
                    "type": "file",
                    "name": "somebody",
                    "value": "/dev/null"
                }
            },
            json! {
                {
                    "type": "color",
                    "name": "somebody",
                    "value": "#FF00FFAA"
                }
            },
            json! {
                {
                    "type": "float",
                    "name": "somebody",
                    "value": 42.42
                }
            },
            json! {
                {
                    "type": "string",
                    "name": "somebody",
                    "value": "onestoldme"
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Property::Int(IntProperty {
                name:  "somebody".to_owned(),
                value: 42,
            }),
            Property::Bool(BoolProperty {
                name:  "somebody".to_owned(),
                value: true,
            }),
            Property::File(FileProperty {
                name:  "somebody".to_owned(),
                value: PathBuf::from("/dev/null"),
            }),
            Property::Color(ColorProperty {
                name:  "somebody".to_owned(),
                value: Color::new(0x00, 0xFF, 0xAA),
            }),
            Property::Float(FloatProperty {
                name:  "somebody".to_owned(),
                value: 42.42,
            }),
            Property::String(StringProperty {
                name:  "somebody".to_owned(),
                value: "onestoldme".to_owned(),
            }),
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
