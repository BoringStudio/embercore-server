use serde::Deserialize;
use serde::Serialize;

use std::str::FromStr;

use crate::tme::error;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

impl FromStr for Orientation {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "orthogonal" => Ok(Orientation::Orthogonal),
            "isometric" => Ok(Orientation::Isometric),
            "staggered" => Ok(Orientation::Staggered),
            "hexagonal" => Ok(Orientation::Hexagonal),
            _ => error::ParseOrientation { s: s.to_owned() }.fail(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::Orientation::{Hexagonal, Isometric, Orthogonal, Staggered};
    use lazy_static::*;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Wrapper {
        orientation: Orientation,
    }

    impl Wrapper {
        pub fn new(orientation: Orientation) -> Self {
            Self { orientation }
        }
    }

    lazy_static! {
        static ref DE_ORIENTATION_STR: String = r#"
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
        "#
        .to_string();
        static ref SER_ORIENTATION_STR: Vec<String> = vec![
            r#"
                {
                    "orientation": "orthogonal"
                }
            "#
            .to_string(),
            r#"
                {
                    "orientation": "isometric"
                }
            "#
            .to_string(),
            r#"
                {
                    "orientation": "staggered"
                }
            "#
            .to_string(),
            r#"
                {
                    "orientation": "hexagonal"
                }
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_orientation() {
        let actuals: Vec<Wrapper> = serde_json::from_str(DE_ORIENTATION_STR.as_str()).unwrap();

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
        let expecteds: Vec<String> = SER_ORIENTATION_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Wrapper::new(Orthogonal)).unwrap(),
            serde_json::to_string(&Wrapper::new(Isometric)).unwrap(),
            serde_json::to_string(&Wrapper::new(Staggered)).unwrap(),
            serde_json::to_string(&Wrapper::new(Hexagonal)).unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
