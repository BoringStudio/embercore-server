use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use super::hexagonal_map::HexagonalMap;
use super::isometric_map::IsometricMap;
use super::orthogonal_map::OrthogonalMap;
use super::staggered_map::StaggeredMap;

use crate::tme::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "orientation", rename_all = "lowercase")]
pub enum Map {
    Orthogonal(OrthogonalMap),
    Isometric(IsometricMap),
    Staggered(StaggeredMap),
    Hexagonal(HexagonalMap),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MapType {
    Map,
}

impl FromStr for MapType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "map" => Ok(MapType::Map),
            _ => Error::ParseMapType(s.to_owned()).fail(),
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "right-down" => Ok(RenderOrder::RightDown),
            "right-up" => Ok(RenderOrder::RightUp),
            "left-down" => Ok(RenderOrder::LeftDown),
            "left-up" => Ok(RenderOrder::LeftUp),
            _ => Error::ParseRenderOrder(s.to_owned()).fail(),
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(StaggerAxis::X),
            "y" => Ok(StaggerAxis::Y),
            _ => Error::ParseStaggerAxis(s.to_owned()).fail(),
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "odd" => Ok(StaggerIndex::Odd),
            "even" => Ok(StaggerIndex::Even),
            _ => Error::ParseStaggerIndex(s.to_owned()).fail(),
        }
    }
}
