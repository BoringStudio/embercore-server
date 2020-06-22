use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_wang_tile() {
        let actuals: Vec<WangTile> = serde_json::from_value(json! {
            [
                {
                    "dflip":  true,
                    "hflip":  false,
                    "tileid": 555,
                    "vflip":  true,
                    "wangid": [56, 789],
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<WangTile> = vec![WangTile {
            d_flip:  true,
            h_flip:  false,
            tile_id: 555,
            v_flip:  true,
            wang_id: vec![56, 789],
        }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_wang_tile() {
        let expecteds: Vec<String> = vec![json! {
            {
                "dflip":  true,
                "hflip":  false,
                "tileid": 555,
                "vflip":  true,
                "wangid": [56, 789],
            }
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![WangTile {
            d_flip:  true,
            h_flip:  false,
            tile_id: 555,
            v_flip:  true,
            wang_id: vec![56, 789],
        }]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
