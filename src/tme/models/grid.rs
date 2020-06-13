use serde::Deserialize;
use serde::Serialize;

use super::orientation::Orientation;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Grid {
    pub height:      i32,
    pub orientation: Orientation,
    pub width:       i32,
}
