use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use super::error::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Color {
    #[serde(default = "default_alpha")]
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn default_alpha() -> u8 {
    u8::max_value()
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
    type Err = Error;

    #[allow(clippy::many_single_char_names)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = if s.starts_with('#') { &s[1..] } else { s };

        let (a, r, g, b) = match s.len() {
            6 => {
                let a = u8::max_value();
                let r = u8::from_str_radix(&s[0..2], 16).map_err(Error::ParseColorComponent)?;
                let g = u8::from_str_radix(&s[2..4], 16).map_err(Error::ParseColorComponent)?;
                let b = u8::from_str_radix(&s[4..6], 16).map_err(Error::ParseColorComponent)?;
                (a, r, g, b)
            }
            8 => {
                let a = u8::from_str_radix(&s[0..2], 16).map_err(Error::ParseColorComponent)?;
                let r = u8::from_str_radix(&s[2..4], 16).map_err(Error::ParseColorComponent)?;
                let g = u8::from_str_radix(&s[4..6], 16).map_err(Error::ParseColorComponent)?;
                let b = u8::from_str_radix(&s[6..8], 16).map_err(Error::ParseColorComponent)?;
                (a, r, g, b)
            }
            _ => {
                return Error::ParseColor(s.to_owned()).fail();
            }
        };

        Ok(Color::with_alpha(a, r, g, b))
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.a, self.r, self.g, self.b)
    }
}

pub mod opt_color_serde {
    use std::str::FromStr;

    use serde::de;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serializer;

    use super::Color;

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
            return Ok(Some(Color::from_str(&s).map_err(de::Error::custom)?));
        }

        Ok(None)
    }
}

pub mod color_serde {
    use std::str::FromStr;

    use serde::de;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serializer;

    use super::Color;

    pub fn serialize<S>(date: &Color, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&*date.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from_str(&s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestColorWrapper {
        #[serde(with = "color_serde")]
        pub color: Color,
    }

    impl TestColorWrapper {
        pub fn new(color: Color) -> Self {
            Self { color }
        }
    }

    #[test]
    fn deserialize_color_from_str() {
        let expected_colors = vec![
            TestColorWrapper::new(Color::new(100, 100, 100)),
            TestColorWrapper::new(Color::new(255, 255, 255)),
            TestColorWrapper::new(Color::new(0, 0, 0)),
            TestColorWrapper::new(Color::new(5, 5, 5)),
            TestColorWrapper::new(Color::with_alpha(255, 100, 100, 100)),
            TestColorWrapper::new(Color::with_alpha(100, 255, 255, 255)),
            TestColorWrapper::new(Color::with_alpha(50, 0, 0, 0)),
            TestColorWrapper::new(Color::with_alpha(0, 5, 5, 5)),
        ];

        let actual_colors: Vec<TestColorWrapper> = serde_json::from_str(
            r##"
        [
            {"color": "#646464"},
            {"color": "#FFFFFF"},
            {"color": "#000000"},
            {"color": "#050505"},
            {"color": "#FF646464"},
            {"color": "#64FFFFFF"},
            {"color": "#32000000"},
            {"color": "#00050505"}
        ]"##,
        )
        .unwrap();

        for (actual, expected) in actual_colors.into_iter().zip(expected_colors) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_color_to_str() {
        let actual_colors = vec![
            serde_json::to_string(&TestColorWrapper::new(Color::new(100, 100, 100))).unwrap(),
            serde_json::to_string(&TestColorWrapper::new(Color::new(255, 255, 255))).unwrap(),
            serde_json::to_string(&TestColorWrapper::new(Color::new(0, 0, 0))).unwrap(),
            serde_json::to_string(&TestColorWrapper::new(Color::new(5, 5, 5))).unwrap(),
            serde_json::to_string(&TestColorWrapper::new(Color::with_alpha(
                255, 100, 100, 100,
            )))
            .unwrap(),
            serde_json::to_string(&TestColorWrapper::new(Color::with_alpha(
                100, 255, 255, 255,
            )))
            .unwrap(),
            serde_json::to_string(&TestColorWrapper::new(Color::with_alpha(50, 0, 0, 0))).unwrap(),
            serde_json::to_string(&TestColorWrapper::new(Color::with_alpha(0, 5, 5, 5))).unwrap(),
        ];

        let expected_colors: Vec<String> = [
            r##"{"color":"#FF646464"}"##.to_owned(),
            r##"{"color":"#FFFFFFFF"}"##.to_owned(),
            r##"{"color":"#FF000000"}"##.to_owned(),
            r##"{"color":"#FF050505"}"##.to_owned(),
            r##"{"color":"#FF646464"}"##.to_owned(),
            r##"{"color":"#64FFFFFF"}"##.to_owned(),
            r##"{"color":"#32000000"}"##.to_owned(),
            r##"{"color":"#00050505"}"##.to_owned(),
        ]
        .iter()
        .map(|s| s.replace(' ', ""))
        .collect();

        for (actual, expected) in actual_colors.into_iter().zip(expected_colors) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn deserialize_color_from_struct() {
        let expected_colors = vec![
            Color::new(100, 100, 100),
            Color::new(255, 255, 255),
            Color::new(0, 0, 0),
            Color::new(5, 5, 5),
            Color::with_alpha(255, 100, 100, 100),
            Color::with_alpha(100, 255, 255, 255),
            Color::with_alpha(50, 0, 0, 0),
            Color::with_alpha(0, 5, 5, 5),
        ];

        let actual_colors: Vec<Color> = serde_json::from_str(
            r##"
        [
            {          "r":100, "g":100, "b":100},
            {          "r":255, "g":255, "b":255},
            {          "r":0,   "g":0,   "b":0},
            {          "r":5,   "g":5,   "b":5},
            { "a":255, "r":100, "g":100, "b":100},
            { "a":100, "r":255, "g":255, "b":255},
            { "a":50,  "r":0,   "g":0,   "b":0},
            { "a":0,   "r":5,   "g":5,   "b":5}
        ]"##,
        )
        .unwrap();

        for (actual, expected) in actual_colors.into_iter().zip(expected_colors) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_color_to_struct() {
        let actual_colors = vec![
            serde_json::to_string(&Color::new(100, 100, 100)).unwrap(),
            serde_json::to_string(&Color::new(255, 255, 255)).unwrap(),
            serde_json::to_string(&Color::new(0, 0, 0)).unwrap(),
            serde_json::to_string(&Color::new(5, 5, 5)).unwrap(),
            serde_json::to_string(&Color::with_alpha(255, 100, 100, 100)).unwrap(),
            serde_json::to_string(&Color::with_alpha(100, 255, 255, 255)).unwrap(),
            serde_json::to_string(&Color::with_alpha(50, 0, 0, 0)).unwrap(),
            serde_json::to_string(&Color::with_alpha(0, 5, 5, 5)).unwrap(),
        ];

        let expected_colors: Vec<String> = [
            r#"{ "a":255, "r":100, "g":100, "b":100}"#.to_owned(),
            r#"{ "a":255, "r":255, "g":255, "b":255}"#.to_owned(),
            r#"{ "a":255, "r":0,   "g":0,   "b":0}"#.to_owned(),
            r#"{ "a":255, "r":5,   "g":5,   "b":5}"#.to_owned(),
            r#"{ "a":255, "r":100, "g":100, "b":100}"#.to_owned(),
            r#"{ "a":100, "r":255, "g":255, "b":255}"#.to_owned(),
            r#"{ "a":50,  "r":0,   "g":0,   "b":0}"#.to_owned(),
            r#"{ "a":0,   "r":5,   "g":5,   "b":5}"#.to_owned(),
        ]
        .iter()
        .map(|s| s.replace(' ', ""))
        .collect();

        for (actual, expected) in actual_colors.into_iter().zip(expected_colors) {
            assert_eq!(actual, expected);
        }
    }
}
