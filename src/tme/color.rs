use super::error;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            a: u8::max_value(),
            r,
            g,
            b,
        }
    }

    pub fn with_alpha(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }
}

impl FromStr for Color {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = if s.starts_with("#") { &s[1..] } else { s };

        let (a, r, g, b) = match s.len() {
            6 => {
                let a = u8::max_value();
                let r = u8::from_str_radix(&s[0..2], 16).context(error::ParseColorComponent)?;
                let g = u8::from_str_radix(&s[2..4], 16).context(error::ParseColorComponent)?;
                let b = u8::from_str_radix(&s[4..6], 16).context(error::ParseColorComponent)?;
                (a, r, g, b)
            }
            8 => {
                let a = u8::from_str_radix(&s[0..2], 16).context(error::ParseColorComponent)?;
                let r = u8::from_str_radix(&s[2..4], 16).context(error::ParseColorComponent)?;
                let g = u8::from_str_radix(&s[4..6], 16).context(error::ParseColorComponent)?;
                let b = u8::from_str_radix(&s[6..8], 16).context(error::ParseColorComponent)?;
                (a, r, g, b)
            }
            _ => {
                return error::ParseColor { s }.fail();
            }
        };

        Ok(Color::with_alpha(a, r, g, b))
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        String::from(format!("#{:X}{:X}{:X}{:X}", self.a, self.r, self.g, self.b))
    }
}

pub mod opt_color_serde {
    use super::Color;
    use serde::de::Error;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(date: &Option<Color>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ref c) = *date {
            return s.serialize_str(&*c.to_string());
        }
        s.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(Some(Color::from_str(&s).map_err(serde::de::Error::custom)?));
        }

        Ok(None)
    }
}
