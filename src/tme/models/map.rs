use super::super::error;
use super::{layer::Layer, property::Property, tileset::Tileset};
use crate::tme::color::{opt_color_serde, Color};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Map {
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
    tile_version:     String,
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
