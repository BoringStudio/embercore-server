use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct TileOffset {
    pub x: i64,
    pub y: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_tile_offset() {
        let actuals: Vec<TileOffset> = serde_json::from_value(json! {
            [
                {
                    "x": 7,
                    "y": 8
                },
                {
                    "x": 77,
                    "y": 66
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<TileOffset> =
            vec![TileOffset { x: 7, y: 8 }, TileOffset { x: 77, y: 66 }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_tile_offset() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "x": 7,
                    "y": 8
                }
            },
            json! {
                {
                    "x": 77,
                    "y": 66
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![TileOffset { x: 7, y: 8 }, TileOffset { x: 77, y: 66 }]
            .into_iter()
            .map(|v| serde_json::to_string(&v).unwrap())
            .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
