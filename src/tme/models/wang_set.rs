use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use super::wang_color::WangColor;
use super::wang_tile::WangTile;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangSet {
    corner_colors: Vec<WangColor>,
    edge_colors:   Vec<WangColor>,
    name:          String,
    properties:    Option<Vec<Property>>,
    tile:          i32,
    wang_tiles:    Vec<WangTile>,
}
