use serde::Deserialize;
use serde::Serialize;

use super::layer::Layer;
use super::map::MapType;
use super::map::RenderOrder;
use super::orientation::Orientation;
use super::property::Property;
use super::tileset::Tileset;
use super::utils;

use crate::tme::color::opt_color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct OrthogonalMap {
    #[serde(with = "opt_color_serde")]
    pub background_color: Option<Color>,
    pub height:           i32,
    pub infinite:         bool,
    pub layers:           Vec<Layer>,
    pub next_layer_id:    i32,
    pub next_object_id:   i32,
    pub orientation:      Orientation,
    pub properties:       Option<Vec<Property>>,
    pub render_order:     RenderOrder,
    pub tiled_version:    String,
    pub tile_height:      i32,
    pub tile_sets:        Vec<Tileset>,
    pub tile_width:       i32,
    #[serde(rename = "type")]
    pub map_type:         MapType,
    #[serde(deserialize_with = "utils::deserialize_value_to_string")]
    pub version:          String,
    pub width:            i32,
}
