use serde::Deserialize;
use serde::Serialize;

use super::grid::Grid;
use super::property::Property;
use super::terrain::Terrain;
use super::tile::Tile;
use super::tile_offset::TileOffset;
use super::utils;
use super::wang_set::WangSet;

use crate::tme::color::opt_color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Tileset {
    #[serde(with = "opt_color_serde")]
    #[serde(rename = "backgroundcolor")]
    pub background_color:  Option<Color>,
    pub columns:           Option<i64>,
    #[serde(rename = "firstgid")]
    pub first_gid:         i64,
    pub grid:              Option<Grid>,
    pub image:             Option<String>,
    #[serde(rename = "imageheight")]
    pub image_height:      Option<i64>,
    #[serde(rename = "imagewidth")]
    pub image_width:       Option<i64>,
    pub margin:            Option<i64>,
    pub name:              String,
    pub properties:        Option<Vec<Property>>,
    pub source:            Option<String>,
    pub spacing:           i64,
    pub terrains:          Option<Vec<Terrain>>,
    #[serde(rename = "tilecount")]
    pub tile_count:        i64,
    #[serde(rename = "tiledversion")]
    pub tiled_version:     String,
    #[serde(rename = "tileheight")]
    pub tile_height:       i64,
    #[serde(rename = "tileoffset")]
    pub tile_offset:       Option<TileOffset>,
    pub tiles:             Option<Vec<Tile>>,
    #[serde(rename = "tilewidth")]
    pub tile_width:        i64,
    #[serde(with = "opt_color_serde")]
    #[serde(rename = "transparentcolor")]
    pub transparent_color: Option<Color>,
    #[serde(rename = "type")]
    pub tileset_type:      String,
    #[serde(deserialize_with = "utils::deserialize_value_to_string")]
    pub version:           String,
    #[serde(rename = "wangsets")]
    pub wang_sets:         Option<Vec<WangSet>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct TilesetRef {
    #[serde(rename = "firstgid")]
    pub first_gid: i64,
    pub source:    String,
}
