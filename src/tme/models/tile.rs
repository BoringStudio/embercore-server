use serde::Deserialize;
use serde::Serialize;

use super::frame::Frame;
use super::layer::Layer;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Tile {
    pub animation:    Option<Vec<Frame>>,
    pub id:           i32,
    pub image:        Option<String>,
    pub image_height: Option<i32>,
    pub image_width:  Option<i32>,
    pub object_group: Option<Layer>,
    pub probability:  Option<f64>,
    pub properties:   Option<Vec<Property>>,
    pub terrain:      Option<Vec<i32>>,
    #[serde(rename = "type")]
    pub tile_type:    Option<String>,
}
