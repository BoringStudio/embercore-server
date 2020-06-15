use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WangTile {
    #[serde(rename = "dflip")]
    pub d_flip:  bool,
    #[serde(rename = "hflip")]
    pub h_flip:  bool,
    #[serde(rename = "tileid")]
    pub tile_id: i64,
    #[serde(rename = "vflip")]
    pub v_flip:  bool,
    #[serde(rename = "wangid")]
    pub wang_id: Vec<i64>,
}
