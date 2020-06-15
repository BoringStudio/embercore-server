use super::data_source::DataSource;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Chunk {
    pub data:   DataSource,
    pub height: i64,
    pub width:  i64,
    pub x:      i64,
    pub y:      i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::data_source::DataSource::{Encoded, Raw};
    use lazy_static::*;

    lazy_static! {
        static ref DE_CHUNKS_STR: String = r#"
            [
                {
                    "data": "qweasdzxcQWEASDZXC",
                    "height": 15,
                    "width": 22,
                    "x": 7,
                    "y": 8
                },
                {
                    "data": [0, 0, 1, 0, 1],
                    "height": 99,
                    "width": 88,
                    "x": 77,
                    "y": 66
                }
            ]
        "#
        .to_string();
        static ref SER_CHUNKS_STR: Vec<String> = vec![
            r#"
                {
                    "data": "qweasdzxcQWEASDZXC",
                    "height": 15,
                    "width": 22,
                    "x": 7,
                    "y": 8
                }
            "#
            .to_string(),
            r#"
                {
                    "data": [0, 0, 1, 0, 1],
                    "height": 99,
                    "width": 88,
                    "x": 77,
                    "y": 66
                }
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_chunk() {
        let actuals: Vec<Chunk> = serde_json::from_str(DE_CHUNKS_STR.as_str()).unwrap();

        let expecteds: Vec<Chunk> = vec![
            Chunk {
                data:   Encoded("qweasdzxcQWEASDZXC".to_owned()),
                height: 15,
                width:  22,
                x:      7,
                y:      8,
            },
            Chunk {
                data:   Raw(vec![0, 0, 1, 0, 1]),
                height: 99,
                width:  88,
                x:      77,
                y:      66,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_chunk() {
        let expecteds: Vec<String> = SER_CHUNKS_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Chunk {
                data:   Encoded("qweasdzxcQWEASDZXC".to_owned()),
                height: 15,
                width:  22,
                x:      7,
                y:      8,
            })
            .unwrap(),
            serde_json::to_string(&Chunk {
                data:   Raw(vec![0, 0, 1, 0, 1]),
                height: 99,
                width:  88,
                x:      77,
                y:      66,
            })
            .unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
