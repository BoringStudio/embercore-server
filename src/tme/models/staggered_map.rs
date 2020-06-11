use serde::Deserialize;
use serde::Serialize;

use super::layer::Layer;
use super::map::MapType;
use super::map::Orientation;
use super::map::RenderOrder;
use super::map::StaggerAxis;
use super::map::StaggerIndex;
use super::property::Property;
use super::tileset::Tileset;

use crate::tme::color::opt_color_serde;
use crate::tme::color::Color;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
