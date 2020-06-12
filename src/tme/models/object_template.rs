use serde::Deserialize;
use serde::Serialize;

use super::object::Object;
use super::tileset::Tileset;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ObjectTemplate {
    #[serde(rename = "type")]
    obj_temp_type: ObjectTemplateType,
    tileset:       Tileset,
    object:        Object,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ObjectTemplateType {
    Template,
}
