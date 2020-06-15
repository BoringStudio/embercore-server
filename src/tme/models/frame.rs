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

    use lazy_static::*;

    lazy_static! {
        static ref DE_FRAMES_STR: String = r#"
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
        "#
        .to_string();
        static ref SER_FRAMES_STR: Vec<String> = vec![
            r#"
                {
                    "duration": 666,
                    "tiledid": 777
                }
            "#
            .to_string(),
            r#"
                {
                    "duration": 7,
                    "tiledid": 6
                }
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_frame() {
        let actuals: Vec<Frame> = serde_json::from_str(DE_FRAMES_STR.as_str()).unwrap();

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
        let expecteds: Vec<String> = SER_FRAMES_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Frame {
                duration: 666,
                tiled_id: 777,
            })
            .unwrap(),
            serde_json::to_string(&Frame {
                duration: 7,
                tiled_id: 6,
            })
            .unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
