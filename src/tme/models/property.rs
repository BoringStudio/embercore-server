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

    use lazy_static::*;

    lazy_static! {
        static ref DE_PROPERTIES_STR: String = r#"
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
        "#
        .to_string();
        static ref SER_PROPERTIES_STR: Vec<String> = vec![
            r#"
                {
                    "type": "int",
                    "name": "somebody",
                    "value": 42
                }
            "#
            .to_string(),
            r#"
                {
                    "type": "bool",
                    "name": "somebody",
                    "value": true
                }
            "#
            .to_string(),
            r#"
                {
                    "type": "file",
                    "name": "somebody",
                    "value": "/dev/null"
                }
            "#
            .to_string(),
            r##"
                {
                    "type": "color",
                    "name": "somebody",
                    "value": "#FF00FFAA"
                }
            "##
            .to_string(),
            r#"
                {
                    "type": "float",
                    "name": "somebody",
                    "value": 42.42
                }
            "#
            .to_string(),
            r#"
                {
                    "type": "string",
                    "name": "somebody",
                    "value": "onestoldme"
                }
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_property() {
        let actuals: Vec<Property> = serde_json::from_str(DE_PROPERTIES_STR.as_str()).unwrap();

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
        let expecteds: Vec<String> = SER_PROPERTIES_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Property::Int(IntProperty {
                name:  "somebody".to_owned(),
                value: 42,
            }))
            .unwrap(),
            serde_json::to_string(&Property::Bool(BoolProperty {
                name:  "somebody".to_owned(),
                value: true,
            }))
            .unwrap(),
            serde_json::to_string(&Property::File(FileProperty {
                name:  "somebody".to_owned(),
                value: PathBuf::from("/dev/null"),
            }))
            .unwrap(),
            serde_json::to_string(&Property::Color(ColorProperty {
                name:  "somebody".to_owned(),
                value: Color::new(0x00, 0xFF, 0xAA),
            }))
            .unwrap(),
            serde_json::to_string(&Property::Float(FloatProperty {
                name:  "somebody".to_owned(),
                value: 42.42,
            }))
            .unwrap(),
            serde_json::to_string(&Property::String(StringProperty {
                name:  "somebody".to_owned(),
                value: "onestoldme".to_owned(),
            }))
            .unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
