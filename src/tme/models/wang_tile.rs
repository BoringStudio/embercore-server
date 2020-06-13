use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangTile {
    pub d_flip:  bool,
    pub h_flip:  bool,
    pub tile_id: i32,
    pub v_flip:  bool,
    pub wang_id: Vec<i32>,
}
