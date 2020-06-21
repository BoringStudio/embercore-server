use serde::Deserialize;
use serde::Serialize;

use std::str::FromStr;

use crate::tme::error::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DataSource {
    Raw(Vec<i64>),
    Encoded(String),
}

impl FromStr for DataSource {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DataSource::Encoded(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::data_source::DataSource::{Encoded, Raw};
    use lazy_static::*;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Wrapper {
        data: DataSource,
    }

    impl Wrapper {
        pub fn new(data: DataSource) -> Self {
            Self { data }
        }
    }

    lazy_static! {
        static ref DE_DATA_SOURCE_STR: String = r#"
            [
                {
                    "data": "qweasdzxcQWEASDZXC"
                },
                {
                    "data": [0, 0, 1, 0, 1]
                }
            ]
        "#
        .to_string();
        static ref SER_DATA_SOURCE_STR: Vec<String> = vec![
            r#"
                {
                    "data": "qweasdzxcQWEASDZXC"
                }
            "#
            .to_string(),
            r#"
                {
                    "data": [0, 0, 1, 0, 1]
                }
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_data_source() {
        let actuals: Vec<Wrapper> = serde_json::from_str(DE_DATA_SOURCE_STR.as_str()).unwrap();

        let expecteds: Vec<Wrapper> = vec![
            Wrapper::new(Encoded("qweasdzxcQWEASDZXC".to_owned())),
            Wrapper::new(Raw(vec![0, 0, 1, 0, 1])),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_data_source() {
        let expecteds: Vec<String> = SER_DATA_SOURCE_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Wrapper::new(Encoded("qweasdzxcQWEASDZXC".to_owned()))).unwrap(),
            serde_json::to_string(&Wrapper::new(Raw(vec![0, 0, 1, 0, 1]))).unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
