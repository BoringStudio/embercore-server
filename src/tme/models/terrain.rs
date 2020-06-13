use serde::Deserialize;
use serde::Serialize;

use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Terrain {
    pub name:       String,
    pub properties: Option<Vec<Property>>,
    pub tile:       i32,
}
