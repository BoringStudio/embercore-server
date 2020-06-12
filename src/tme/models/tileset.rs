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
    background_color:  Option<Color>,
    columns:           i32,
    first_gid:         i32,
    grid:              Option<Grid>,
    image:             String,
    image_height:      i32,
    image_width:       i32,
    margin:            i32,
    name:              String,
    properties:        Option<Vec<Property>>,
    source:            Option<String>,
    spacing:           i32,
    terrains:          Option<Vec<Terrain>>,
    tile_count:        i32,
    tiled_version:     String,
    tile_height:       i32,
    tile_offset:       Option<TileOffset>,
    tiles:             Option<Vec<Tile>>,
    tile_width:        i32,
    transparent_color: Option<Color>,
    #[serde(rename = "type")]
    tileset_type:      String,
    #[serde(deserialize_with = "utils::deserialize_value_to_string")]
    version:           String,
    wang_sets:         Vec<WangSet>,
}
