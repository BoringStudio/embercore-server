use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use super::group_layer::GroupLayer;
use super::image_layer::ImageLayer;
use super::object_group_layer::ObjectGroupLayer;
use super::tile_layer::TileLayer;
use crate::tme::error::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Layer {
    #[serde(rename = "tilelayer")]
    TileLayer(TileLayer),
    #[serde(rename = "objectgroup")]
    ObjectGroupLayer(ObjectGroupLayer),
    #[serde(rename = "imagelayer")]
    ImageLayer(ImageLayer),
    #[serde(rename = "group")]
    GroupLayer(GroupLayer),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    Zlib,
    Gzip,
}

impl FromStr for Compression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zlib" => Ok(Compression::Zlib),
            "gzip" => Ok(Compression::Gzip),
            _ => Error::ParseCompression(s.to_owned()).fail(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DrawOrder {
    TopDown,
    Index,
}

impl FromStr for DrawOrder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "topdown" => Ok(DrawOrder::TopDown),
            "index" => Ok(DrawOrder::Index),
            _ => Error::ParseDrawOrder(s.to_owned()).fail(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    Csv,
    Base64,
}

impl FromStr for Encoding {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "csv" => Ok(Encoding::Csv),
            "base64" => Ok(Encoding::Base64),
            _ => Error::ParseDrawOrder(s.to_owned()).fail(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerType {
    TileLayer,
    ObjectGroup,
    ImageLayer,
    Group,
}

impl FromStr for LayerType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tilelayer" => Ok(LayerType::TileLayer),
            "objectgroup" => Ok(LayerType::ObjectGroup),
            "imagelayer" => Ok(LayerType::ImageLayer),
            "group" => Ok(LayerType::Group),
            _ => Error::ParseLayerType(s.to_owned()).fail(),
        }
    }
}
