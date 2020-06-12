use serde::Deserialize;
use serde::Serialize;

use super::ellipse_object::EllipseObject;
use super::general_object::GeneralObject;
use super::point_object::PointObject;
use super::polygon_object::PolygonObject;
use super::polyline_object::PolylineObject;
use super::rectangle_object::RectangleObject;
use super::text_object::TextObject;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum Object {
    General(GeneralObject),
    Ellipse(EllipseObject),
    Rectangle(RectangleObject),
    Point(PointObject),
    Polygon(PolygonObject),
    Polyline(PolylineObject),
    Text(TextObject),
}
