use serde::Deserialize;
use serde::Serialize;

use super::orientation::Orientation;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct Grid {
    pub height:      i64,
    pub orientation: Orientation,
    pub width:       i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::*;

    lazy_static! {
        static ref DE_GRIDS_STR: String = r#"
            [
                {
                    "height": 666,
                    "orientation": "orthogonal",
                    "width": 777
                },
                {
                    "height": 666,
                    "orientation": "isometric",
                    "width": 777
                },
                {
                    "height": 666,
                    "orientation": "staggered",
                    "width": 777
                },
                {
                    "height": 666,
                    "orientation": "hexagonal",
                    "width": 777
                }
            ]
        "#
        .to_string();
        static ref SER_GRIDS_STR: Vec<String> = vec![
            r#"
                {
                    "height": 666,
                    "orientation": "orthogonal",
                    "width": 777
                }
            "#
            .to_string(),
            r#"
                {
                    "height": 666,
                    "orientation": "isometric",
                    "width": 777
                }
            "#
            .to_string(),
            r#"
                {
                    "height": 666,
                    "orientation": "staggered",
                    "width": 777
                }
            "#
            .to_string(),
            r#"
                {
                    "height": 666,
                    "orientation": "hexagonal",
                    "width": 777
                }
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_grid() {
        let actuals: Vec<Grid> = serde_json::from_str(DE_GRIDS_STR.as_str()).unwrap();

        let expecteds: Vec<Grid> = vec![
            Grid {
                height:      666,
                orientation: Orientation::Orthogonal,
                width:       777,
            },
            Grid {
                height:      666,
                orientation: Orientation::Isometric,
                width:       777,
            },
            Grid {
                height:      666,
                orientation: Orientation::Staggered,
                width:       777,
            },
            Grid {
                height:      666,
                orientation: Orientation::Hexagonal,
                width:       777,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_grid() {
        let expecteds: Vec<String> = SER_GRIDS_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Grid {
                height:      666,
                orientation: Orientation::Orthogonal,
                width:       777,
            })
            .unwrap(),
            serde_json::to_string(&Grid {
                height:      666,
                orientation: Orientation::Isometric,
                width:       777,
            })
            .unwrap(),
            serde_json::to_string(&Grid {
                height:      666,
                orientation: Orientation::Staggered,
                width:       777,
            })
            .unwrap(),
            serde_json::to_string(&Grid {
                height:      666,
                orientation: Orientation::Hexagonal,
                width:       777,
            })
            .unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
