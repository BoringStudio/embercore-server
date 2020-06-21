use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct Frame {
    pub duration: i64,
    #[serde(rename = "tiledid")]
    pub tiled_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_frame() {
        let actuals: Vec<Frame> = serde_json::from_value(json! {
            [
                {
                    "duration": 666,
                    "tiledid": 777
                },
                {
                    "duration": 7,
                    "tiledid": 6
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Frame> = vec![
            Frame {
                duration: 666,
                tiled_id: 777,
            },
            Frame {
                duration: 7,
                tiled_id: 6,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_frame() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "duration": 666,
                    "tiledid": 777
                }
            },
            json! {
                {
                    "duration": 7,
                    "tiledid": 6
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Frame {
                duration: 666,
                tiled_id: 777,
            },
            Frame {
                duration: 7,
                tiled_id: 6,
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
