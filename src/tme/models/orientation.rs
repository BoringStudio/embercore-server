use serde::Deserialize;
use serde::Serialize;

use std::str::FromStr;

use crate::tme::error::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

impl FromStr for Orientation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "orthogonal" => Ok(Orientation::Orthogonal),
            "isometric" => Ok(Orientation::Isometric),
            "staggered" => Ok(Orientation::Staggered),
            "hexagonal" => Ok(Orientation::Hexagonal),
            _ => Error::ParseOrientation(s.to_owned()).fail(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::Orientation::{Hexagonal, Isometric, Orthogonal, Staggered};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Wrapper {
        orientation: Orientation,
    }

    impl Wrapper {
        pub fn new(orientation: Orientation) -> Self {
            Self { orientation }
        }
    }

    #[test]
    fn deserialize_orientation() {
        let actuals: Vec<Wrapper> = serde_json::from_value(json! {
            [
                {
                    "orientation": "orthogonal"
                },
                {
                    "orientation": "isometric"
                },
                {
                    "orientation": "staggered"
                },
                {
                    "orientation": "hexagonal"
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Wrapper> = vec![
            Wrapper::new(Orthogonal),
            Wrapper::new(Isometric),
            Wrapper::new(Staggered),
            Wrapper::new(Hexagonal),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_orientation() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "orientation": "orthogonal"
                }
            },
            json! {
                {
                    "orientation": "isometric"
                }
            },
            json! {
                {
                    "orientation": "staggered"
                }
            },
            json! {
                {
                    "orientation": "hexagonal"
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Wrapper::new(Orthogonal),
            Wrapper::new(Isometric),
            Wrapper::new(Staggered),
            Wrapper::new(Hexagonal),
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
