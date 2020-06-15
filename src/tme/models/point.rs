use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::*;

    lazy_static! {
        static ref DE_POINTS_STR: String = r#"
            [
                {
                    "x": 7.7,
                    "y": 8.8
                },
                {
                    "x": 77.77,
                    "y": 66.66
                }
            ]
        "#
        .to_string();
        static ref SER_POINTS_STR: Vec<String> = vec![
            r#"
                {
                    "x": 7.7,
                    "y": 8.8
                }
            "#
            .to_string(),
            r#"
                {
                    "x": 77.77,
                    "y": 66.66
                }
            "#
            .to_string(),
        ]
        .into_iter()
        .map(|s| s.replace(' ', "").replace('\n', ""))
        .collect();
    }

    #[test]
    fn deserialize_point() {
        let actuals: Vec<Point> = serde_json::from_str(DE_POINTS_STR.as_str()).unwrap();

        let expecteds: Vec<Point> = vec![Point { x: 7.7, y: 8.8 }, Point { x: 77.77, y: 66.66 }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_point() {
        let expecteds: Vec<String> = SER_POINTS_STR.to_vec();

        let actuals: Vec<String> = vec![
            serde_json::to_string(&Point { x: 7.7, y: 8.8 }).unwrap(),
            serde_json::to_string(&Point { x: 77.77, y: 66.66 }).unwrap(),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
