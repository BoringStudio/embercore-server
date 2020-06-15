use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Property {
    Int { name: String, value: i64 },
    Bool { name: String, value: bool },
    File { name: String, value: String },
    Color { name: String, value: Color },
    Float { name: String, value: f64 },
    String { name: String, value: String },
}
