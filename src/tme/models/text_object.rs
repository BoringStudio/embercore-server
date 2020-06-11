use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use super::text::Text;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct TextObject {
    height:     f64,
    id:         i32,
    name:       String,
    properties: Option<Vec<Property>>,
    rotation:   f64,
    template:   Option<String>,
    text:       Text,
    #[serde(rename = "type")]
    obj_type:   String,
    visible:    bool,
    width:      f64,
    x:          f64,
    y:          f64,
}
