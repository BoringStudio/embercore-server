use serde::Deserialize;
use serde::Serialize;

use super::object::Object;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ObjectTemplate {
    #[serde(rename = "type")]
    pub obj_temp_type: ObjectTemplateType,
    pub tileset:       Option<ExternalTileset>,
    pub object:        Object,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ObjectTemplateType {
    Template,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ExternalTileset {
    pub first_gid: i64,
    pub source:    String,
}
