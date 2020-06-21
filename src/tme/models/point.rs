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

    use serde_json::json;

    #[test]
    fn deserialize_point() {
        let actuals: Vec<Point> = serde_json::from_value(json! {
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
        })
        .unwrap();

        let expecteds: Vec<Point> = vec![Point { x: 7.7, y: 8.8 }, Point { x: 77.77, y: 66.66 }];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_point() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "x": 7.7,
                    "y": 8.8
                }
            },
            json! {
                {
                    "x": 77.77,
                    "y": 66.66
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![Point { x: 7.7, y: 8.8 }, Point { x: 77.77, y: 66.66 }]
            .into_iter()
            .map(|v| serde_json::to_string(&v).unwrap())
            .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
