use super::super::error;
use super::{layer::Layer, property::Property, tileset::Tileset};
use crate::tme::color::{opt_color_serde, Color};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Map {
    Orthogonal(OrthogonalMap),
    Isometric(IsometricMap),
    Staggered(StaggeredMap),
    Hexagonal(HexagonalMap),
}

impl<'de> Deserialize<'de> for Map {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "lowercase", tag = "orientation")]
        enum OrientationHelper {
            Orthogonal,
            Isometric,
            Staggered,
            Hexagonal,
        }

        let v = serde_json::Value::deserialize(deserializer)?;
        let o = OrientationHelper::deserialize(&v).map_err(de::Error::custom)?;
        match o {
            OrientationHelper::Orthogonal => Ok(Map::Orthogonal(
                OrthogonalMap::deserialize(&v).map_err(de::Error::custom)?,
            )),
            OrientationHelper::Isometric => Ok(Map::Isometric(
                IsometricMap::deserialize(&v).map_err(de::Error::custom)?,
            )),
            OrientationHelper::Staggered => Ok(Map::Staggered(
                StaggeredMap::deserialize(&v).map_err(de::Error::custom)?,
            )),
            OrientationHelper::Hexagonal => Ok(Map::Hexagonal(
                HexagonalMap::deserialize(&v).map_err(de::Error::custom)?,
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct OrthogonalMap {
    #[serde(with = "opt_color_serde")]
    background_color: Option<Color>,
    height:           i32,
    infinite:         bool,
    layers:           Vec<Layer>,
    next_layer_id:    i32,
    next_object_id:   i32,
    orientation:      Orientation,
    properties:       Vec<Property>,
    render_order:     RenderOrder,
    tiled_version:    String,
    tile_height:      i32,
    tile_sets:        Vec<Tileset>,
    tile_width:       i32,
    #[serde(rename = "type")]
    map_type:         MapType,
    version:          serde_json::Number,
    width:            i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct IsometricMap {
    #[serde(with = "opt_color_serde")]
    background_color: Option<Color>,
    height:           i32,
    infinite:         bool,
    layers:           Vec<Layer>,
    next_layer_id:    i32,
    next_object_id:   i32,
    orientation:      Orientation,
    properties:       Vec<Property>,
    render_order:     RenderOrder,
    tiled_version:    String,
    tile_height:      i32,
    tile_sets:        Vec<Tileset>,
    tile_width:       i32,
    #[serde(rename = "type")]
    map_type:         MapType,
    version:          serde_json::Number,
    width:            i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct StaggeredMap {
    #[serde(with = "opt_color_serde")]
    background_color: Option<Color>,
    height:           i32,
    infinite:         bool,
    layers:           Vec<Layer>,
    next_layer_id:    i32,
    next_object_id:   i32,
    orientation:      Orientation,
    properties:       Vec<Property>,
    render_order:     RenderOrder,
    stagger_axis:     StaggerAxis,
    stagger_index:    StaggerIndex,
    tiled_version:    String,
    tile_height:      i32,
    tile_sets:        Vec<Tileset>,
    tile_width:       i32,
    #[serde(rename = "type")]
    map_type:         MapType,
    version:          serde_json::Number,
    width:            i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct HexagonalMap {
    #[serde(with = "opt_color_serde")]
    background_color: Option<Color>,
    height:           i32,
    hex_side_length:  i32,
    infinite:         bool,
    layers:           Vec<Layer>,
    next_layer_id:    i32,
    next_object_id:   i32,
    orientation:      Orientation,
    properties:       Vec<Property>,
    render_order:     RenderOrder,
    stagger_axis:     StaggerAxis,
    stagger_index:    StaggerIndex,
    tiled_version:    String,
    tile_height:      i32,
    tile_sets:        Vec<Tileset>,
    tile_width:       i32,
    #[serde(rename = "type")]
    map_type:         MapType,
    version:          serde_json::Number,
    width:            i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MapType {
    Map,
}

impl FromStr for MapType {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "map" => Ok(MapType::Map),
            _ => error::ParseMapType { s: s.to_owned() }.fail(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

impl FromStr for Orientation {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "orthogonal" => Ok(Orientation::Orthogonal),
            "isometric" => Ok(Orientation::Isometric),
            "staggered" => Ok(Orientation::Staggered),
            "hexagonal" => Ok(Orientation::Hexagonal),
            _ => error::ParseOrientation { s: s.to_owned() }.fail(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl FromStr for RenderOrder {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "right-down" => Ok(RenderOrder::RightDown),
            "right-up" => Ok(RenderOrder::RightUp),
            "left-down" => Ok(RenderOrder::LeftDown),
            "left-up" => Ok(RenderOrder::LeftUp),
            _ => error::ParseRenderOrder { s: s.to_owned() }.fail(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaggerAxis {
    X,
    Y,
}

impl FromStr for StaggerAxis {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(StaggerAxis::X),
            "y" => Ok(StaggerAxis::Y),
            _ => error::ParseStaggerAxis { s: s.to_owned() }.fail(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaggerIndex {
    Odd,
    Even,
}

impl FromStr for StaggerIndex {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "odd" => Ok(StaggerIndex::Odd),
            "even" => Ok(StaggerIndex::Even),
            _ => error::ParseStaggerIndex { s: s.to_owned() }.fail(),
        }
    }
}
