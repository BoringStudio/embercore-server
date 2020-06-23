use serde::Deserialize;
use serde::Serialize;

use std::path::PathBuf;

use super::grid::Grid;
use super::property::Property;
use super::terrain::Terrain;
use super::tile::Tile;
use super::tile_offset::TileOffset;
use super::utils;
use super::wang_set::WangSet;

use crate::tme::color::opt_color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", untagged)]
pub enum TilesetContainer {
    Tileset(Tileset),
    TilesetRef(TilesetRef),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Tileset {
    #[serde(with = "opt_color_serde")]
    #[serde(rename = "backgroundcolor")]
    #[serde(default = "utils::make_none_option")]
    pub background_color:  Option<Color>,
    #[serde(default = "utils::make_none_option")]
    pub columns:           Option<i64>,
    #[serde(default = "utils::make_none_option")]
    pub grid:              Option<Grid>,
    #[serde(default = "utils::make_none_option")]
    pub image:             Option<String>,
    #[serde(rename = "imageheight")]
    #[serde(default = "utils::make_none_option")]
    pub image_height:      Option<i64>,
    #[serde(rename = "imagewidth")]
    #[serde(default = "utils::make_none_option")]
    pub image_width:       Option<i64>,
    #[serde(default = "utils::make_none_option")]
    pub margin:            Option<i64>,
    pub name:              String,
    #[serde(default = "utils::make_none_option")]
    pub properties:        Option<Vec<Property>>,
    pub spacing:           i64,
    #[serde(default = "utils::make_none_option")]
    pub terrains:          Option<Vec<Terrain>>,
    #[serde(rename = "tilecount")]
    pub tile_count:        i64,
    #[serde(rename = "tiledversion")]
    pub tiled_version:     String,
    #[serde(rename = "tileheight")]
    pub tile_height:       i64,
    #[serde(rename = "tileoffset")]
    #[serde(default = "utils::make_none_option")]
    pub tile_offset:       Option<TileOffset>,
    #[serde(default = "utils::make_none_option")]
    pub tiles:             Option<Vec<Tile>>,
    #[serde(rename = "tilewidth")]
    pub tile_width:        i64,
    #[serde(with = "opt_color_serde")]
    #[serde(rename = "transparentcolor")]
    #[serde(default = "utils::make_none_option")]
    pub transparent_color: Option<Color>,
    #[serde(rename = "type")]
    pub tileset_type:      String,
    #[serde(deserialize_with = "utils::deserialize_value_to_string")]
    pub version:           String,
    #[serde(rename = "wangsets")]
    #[serde(default = "utils::make_none_option")]
    pub wang_sets:         Option<Vec<WangSet>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct TilesetRef {
    #[serde(rename = "firstgid")]
    pub first_gid: i64,
    pub source:    PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_tileset() {
        let actuals: Vec<Tileset> = serde_json::from_value(json! {
            [
                {
                    "backgroundcolor":  null,
                    "columns":          null,
                    "grid":             null,
                    "image":            null,
                    "imageheight":      null,
                    "imagewidth":       null,
                    "margin":           null,
                    "name":             "",
                    "properties":       null,
                    "spacing":          0,
                    "terrains":         null,
                    "tilecount":        0,
                    "tiledversion":     "1.3.5",
                    "tileheight":       0,
                    "tileoffset":       null,
                    "tiles":            null,
                    "tilewidth":        0,
                    "transparentcolor": null,
                    "type":             "",
                    "version":          1.2,
                    "wangsets":         null
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Tileset> = vec![Tileset {
            background_color:  None,
            columns:           None,
            grid:              None,
            image:             None,
            image_height:      None,
            image_width:       None,
            margin:            None,
            name:              "".to_string(),
            properties:        None,
            spacing:           0,
            terrains:          None,
            tile_count:        0,
            tiled_version:     "1.3.5".to_string(),
            tile_height:       0,
            tile_offset:       None,
            tiles:             None,
            tile_width:        0,
            transparent_color: None,
            tileset_type:      "".to_string(),
            version:           "1.2".to_string(),
            wang_sets:         None,
        }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_tileset() {
        let expecteds: Vec<String> = vec![json! {
            {
                "backgroundcolor":  null,
                "columns":          null,
                "grid":             null,
                "image":            null,
                "imageheight":      null,
                "imagewidth":       null,
                "margin":           null,
                "name":             "",
                "properties":       null,
                "spacing":          0,
                "terrains":         null,
                "tilecount":        0,
                "tiledversion":     "1.3.5",
                "tileheight":       0,
                "tileoffset":       null,
                "tiles":            null,
                "tilewidth":        0,
                "transparentcolor": null,
                "type":             "",
                "version":          "1.2",
                "wangsets":         null
            }
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![Tileset {
            background_color:  None,
            columns:           None,
            grid:              None,
            image:             None,
            image_height:      None,
            image_width:       None,
            margin:            None,
            name:              "".to_string(),
            properties:        None,
            spacing:           0,
            terrains:          None,
            tile_count:        0,
            tiled_version:     "1.3.5".to_string(),
            tile_height:       0,
            tile_offset:       None,
            tiles:             None,
            tile_width:        0,
            transparent_color: None,
            tileset_type:      "".to_string(),
            version:           "1.2".to_string(),
            wang_sets:         None,
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
