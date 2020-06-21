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
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Wrapper {
        data: DataSource,
    }

    impl Wrapper {
        pub fn new(data: DataSource) -> Self {
            Self { data }
        }
    }

    #[test]
    fn deserialize_data_source() {
        let actuals: Vec<Wrapper> = serde_json::from_value(json! {
            [
                {
                    "data": "qweasdzxcQWEASDZXC"
                },
                {
                    "data": [0, 0, 1, 0, 1]
                }
            ]
        })
        .unwrap();

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
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "data": "qweasdzxcQWEASDZXC"
                }
            },
            json! {
                {
                    "data": [0, 0, 1, 0, 1]
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Wrapper::new(Encoded("qweasdzxcQWEASDZXC".to_owned())),
            Wrapper::new(Raw(vec![0, 0, 1, 0, 1])),
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
