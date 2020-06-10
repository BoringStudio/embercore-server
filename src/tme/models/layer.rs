use super::super::{color::Color, error};
use super::{chunk::Chunk, object::Object, property::Property, redata::ReData};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Layer {
    chunks:            Vec<Chunk>,
    compression:       Compression,
    data:              Option<ReData>,
    draw_order:        Option<DrawOrder>,
    encoding:          Option<Encoding>,
    height:            i32,
    id:                i32,
    image:             Option<String>,
    layers:            Option<Vec<Layer>>,
    name:              String,
    objects:           Option<Vec<Object>>,
    offset_x:          f64,
    offset_y:          f64,
    opacity:           f64,
    properties:        Vec<Property>,
    start_x:           i32,
    start_y:           i32,
    transparent_color: Option<Color>,
    #[serde(rename = "type")]
    layer_type:        LayerType,
    visible:           bool,
    width:             i32,
    x:                 i32,
    y:                 i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    Zlib,
    Gzip,
    Empty,
}

impl FromStr for Compression {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zlib" => Ok(Compression::Zlib),
            "gzip" => Ok(Compression::Gzip),
            "" => Ok(Compression::Empty),
            _ => error::ParseCompression { s: s.to_owned() }.fail(),
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
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "topdown" => Ok(DrawOrder::TopDown),
            "index" => Ok(DrawOrder::Index),
            _ => error::ParseDrawOrder { s: s.to_owned() }.fail(),
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
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "csv" => Ok(Encoding::Csv),
            "base64" => Ok(Encoding::Base64),
            _ => error::ParseDrawOrder { s: s.to_owned() }.fail(),
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
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tilelayer" => Ok(LayerType::TileLayer),
            "objectgroup" => Ok(LayerType::ObjectGroup),
            "imagelayer" => Ok(LayerType::ImageLayer),
            "group" => Ok(LayerType::Group),
            _ => error::ParseLayerType { s: s.to_owned() }.fail(),
        }
    }
}
