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

    use serde_json::json;

    #[test]
    fn deserialize_grid() {
        let actuals: Vec<Grid> = serde_json::from_value(json! {
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
        })
        .unwrap();

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
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "height": 666,
                    "orientation": "orthogonal",
                    "width": 777
                }
            },
            json! {
                {
                    "height": 666,
                    "orientation": "isometric",
                    "width": 777
                }
            },
            json! {
                {
                    "height": 666,
                    "orientation": "staggered",
                    "width": 777
                }
            },
            json! {
                {
                    "height": 666,
                    "orientation": "hexagonal",
                    "width": 777
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
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
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
