use serde::Deserialize;
use serde::Serialize;

use super::layer::Layer;
use super::map::MapType;
use super::map::RenderOrder;
use super::orientation::Orientation;
use super::property::Property;
use super::tileset::Tileset;
use super::tileset::TilesetRef;
use super::utils;

use crate::tme::color::opt_color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct OrthogonalMap {
    #[serde(with = "opt_color_serde")]
    #[serde(rename = "backgroundcolor")]
    pub background_color: Option<Color>,
    pub height:           i64,
    pub infinite:         bool,
    pub layers:           Vec<Layer>,
    #[serde(rename = "nextlayerid")]
    pub next_layer_id:    i64,
    #[serde(rename = "nextobjectid")]
    pub next_object_id:   i64,
    pub orientation:      Orientation,
    pub properties:       Option<Vec<Property>>,
    #[serde(rename = "renderorder")]
    pub render_order:     RenderOrder,
    #[serde(rename = "tiledversion")]
    pub tiled_version:    String,
    #[serde(rename = "tileheight")]
    pub tile_height:      i64,
    #[serde(rename = "tilesets")]
    pub tile_sets:        Vec<TilesetRef>,
    #[serde(rename = "tilewidth")]
    pub tile_width:       i64,
    #[serde(rename = "type")]
    pub map_type:         MapType,
    #[serde(deserialize_with = "utils::deserialize_value_to_string")]
    pub version:          String,
    pub width:            i64,
}
