use serde::Deserialize;
use serde::Serialize;

use super::property::Property;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Terrain {
    pub name:       String,
    pub properties: Option<Vec<Property>>,
    pub tile:       i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::property::FileProperty;
    use serde_json::json;
    use std::path::PathBuf;

    #[test]
    fn deserialize_terrain() {
        let actuals: Vec<Terrain> = serde_json::from_value(json! {
            [
                {
                    "name": "somebody",
                    "tile": 42
                },
                {
                    "name": "somebody",
                    "properties": [
                        {
                            "type": "file",
                            "name": "onestoldme",
                            "value": "/dev/null"
                        }
                    ],
                    "tile": 42
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Terrain> = vec![
            Terrain {
                name:       "somebody".to_owned(),
                properties: None,
                tile:       42,
            },
            Terrain {
                name:       "somebody".to_owned(),
                properties: Some(vec![Property::File(FileProperty {
                    name:  "onestoldme".to_owned(),
                    value: PathBuf::from("/dev/null"),
                })]),
                tile:       42,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_terrain() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "name": "somebody",
                    "properties": null,
                    "tile": 42
                }
            },
            json! {
                {
                    "name": "somebody",
                    "properties": [
                        {
                            "type": "file",
                            "name": "onestoldme",
                            "value": "/dev/null"
                        }
                    ],
                    "tile": 42
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Terrain {
                name:       "somebody".to_owned(),
                properties: None,
                tile:       42,
            },
            Terrain {
                name:       "somebody".to_owned(),
                properties: Some(vec![Property::File(FileProperty {
                    name:  "onestoldme".to_owned(),
                    value: PathBuf::from("/dev/null"),
                })]),
                tile:       42,
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
