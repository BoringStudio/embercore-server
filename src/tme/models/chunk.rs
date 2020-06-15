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
        static ref DE_CHUNKS_STR: String = String::from(
            r#"
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
        )
        .replace(' ', "");
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
}
