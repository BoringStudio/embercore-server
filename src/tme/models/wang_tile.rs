use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangTile {
    d_flip:  bool,
    h_flip:  bool,
    tile_id: i32,
    v_flip:  bool,
    wang_id: Vec<i32>,
}
