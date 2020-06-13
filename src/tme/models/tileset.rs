use serde::Deserialize;
use serde::Serialize;

use super::grid::Grid;
use super::property::Property;
use super::terrain::Terrain;
use super::tile::Tile;
use super::tile_offset::TileOffset;
use super::utils;
use super::wang_set::WangSet;

use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Tileset {
    pub background_color:  Option<Color>,
    pub columns:           i32,
    pub first_gid:         i32,
    pub grid:              Option<Grid>,
    pub image:             String,
    pub image_height:      i32,
    pub image_width:       i32,
    pub margin:            i32,
    pub name:              String,
    pub properties:        Option<Vec<Property>>,
    pub source:            Option<String>,
    pub spacing:           i32,
    pub terrains:          Option<Vec<Terrain>>,
    pub tile_count:        i32,
    pub tiled_version:     String,
    pub tile_height:       i32,
    pub tile_offset:       Option<TileOffset>,
    pub tiles:             Option<Vec<Tile>>,
    pub tile_width:        i32,
    pub transparent_color: Option<Color>,
    #[serde(rename = "type")]
    pub tileset_type:      String,
    #[serde(deserialize_with = "utils::deserialize_value_to_string")]
    pub version:           String,
    pub wang_sets:         Vec<WangSet>,
}
