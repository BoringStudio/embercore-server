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
    use lazy_static::*;
    use std::path::PathBuf;

    lazy_static! {
        static ref DE_TERRAINS_STR: String = r#"
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
        "#
        .to_string();
        static ref SER_TERRAINS_STR: Vec<String> = vec![
            r#"
                {
                    "name": "somebody",
                    "properties": null,
                    "tile": 42
                }
            "#
            .to_string(),
            r#"
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
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_terrain() {
        let actuals: Vec<Terrain> = serde_json::from_str(DE_TERRAINS_STR.as_str()).unwrap();

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
        let expecteds: Vec<String> = SER_TERRAINS_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Terrain {
                name:       "somebody".to_owned(),
                properties: None,
                tile:       42,
            })
            .unwrap(),
            serde_json::to_string(&Terrain {
                name:       "somebody".to_owned(),
                properties: Some(vec![Property::File(FileProperty {
                    name:  "onestoldme".to_owned(),
                    value: PathBuf::from("/dev/null"),
                })]),
                tile:       42,
            })
            .unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
