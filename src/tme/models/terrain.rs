use serde::Deserialize;
use serde::Serialize;

use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Terrain {
    name:       String,
    properties: Option<Vec<Property>>,
    tile:       i32,
}
