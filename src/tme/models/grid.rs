use serde::Deserialize;
use serde::Serialize;

use super::orientation::Orientation;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Grid {
    height:      i32,
    orientation: Orientation,
    width:       i32,
}
