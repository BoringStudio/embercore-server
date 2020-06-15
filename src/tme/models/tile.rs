use serde::Deserialize;
use serde::Serialize;

use super::frame::Frame;
use super::layer::Layer;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Tile {
    pub animation:    Option<Vec<Frame>>,
    pub id:           i64,
    pub image:        Option<String>,
    #[serde(rename = "imageheight")]
    pub image_height: Option<i64>,
    #[serde(rename = "imagewidth")]
    pub image_width:  Option<i64>,
    #[serde(rename = "objectgroup")]
    pub object_group: Option<Layer>,
    pub probability:  Option<f64>,
    pub properties:   Option<Vec<Property>>,
    pub terrain:      Option<Vec<i64>>,
    #[serde(rename = "type")]
    pub tile_type:    Option<String>,
}

impl Tile {
    pub fn new(id: i64) -> Self {
        Self {
            animation: None,
            id,
            image: None,
            image_height: None,
            image_width: None,
            object_group: None,
            probability: None,
            properties: None,
            terrain: None,
            tile_type: None,
        }
    }
}
