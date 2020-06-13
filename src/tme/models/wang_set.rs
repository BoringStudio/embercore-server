use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use super::wang_color::WangColor;
use super::wang_tile::WangTile;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangSet {
    pub corner_colors: Vec<WangColor>,
    pub edge_colors:   Vec<WangColor>,
    pub name:          String,
    pub properties:    Option<Vec<Property>>,
    pub tile:          i32,
    pub wang_tiles:    Vec<WangTile>,
}
