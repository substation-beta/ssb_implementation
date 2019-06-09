// Imports
use super::ssb::Point2D;


// Enums
#[derive(Debug)]
pub enum EventGeometry {
    Shape(Vec<ShapeSegment>),
    Points(Vec<Point2D>),
    Text(String)
}
#[derive(Debug)]
pub enum ShapeSegment {
    MoveTo(Point2D),
    LineTo(Point2D),
    CurveTo(Point2D, Point2D, Point2D),
    ArcBy(Point2D, f64),
    Close
}