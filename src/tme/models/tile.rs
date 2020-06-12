use serde::Deserialize;
use serde::Serialize;

use super::frame::Frame;
use super::layer::Layer;
use super::property::Property;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Tile {
    animation:    Vec<Frame>,
    id:           i32,
    image:        Option<String>,
    image_height: i32,
    image_width:  i32,
    object_group: Option<Layer>,
    probability:  Option<f64>,
    properties:   Option<Vec<Property>>,
    terrain:      Option<Vec<i32>>,
    #[serde(rename = "type")]
    tile_type:    Option<String>,
}
