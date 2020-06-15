use serde::Deserialize;
use serde::Serialize;

use super::property::Property;
use super::wang_color::WangColor;
use super::wang_tile::WangTile;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangSet {
    #[serde(rename = "cornercolors")]
    pub corner_colors: Vec<WangColor>,
    #[serde(rename = "edgecolors")]
    pub edge_colors:   Vec<WangColor>,
    pub name:          String,
    pub properties:    Option<Vec<Property>>,
    pub tile:          i64,
    #[serde(rename = "wangtiles")]
    pub wang_tiles:    Vec<WangTile>,
}
