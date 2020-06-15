use serde::Deserialize;
use serde::Serialize;

use std::str::FromStr;

use crate::tme::error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DataSource {
    Raw(Vec<i64>),
    Encoded(String),
}

impl FromStr for DataSource {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DataSource::Encoded(s.to_owned()))
    }
}
