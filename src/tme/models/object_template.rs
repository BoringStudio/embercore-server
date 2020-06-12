use serde::Deserialize;
use serde::Serialize;

use super::object::Object;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ObjectTemplate {
    #[serde(rename = "type")]
    obj_temp_type: ObjectTemplateType,
    tileset:       Option<ExternalTileset>,
    object:        Object,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ObjectTemplateType {
    Template,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ExternalTileset {
    first_gid: i32,
    source:    String,
}
