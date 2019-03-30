// Point of path segment
pub struct Point {
    x: f32,
    y: f32
}

// Segment of path
pub enum PathSegment {
    MoveTo(Point),
    LineTo(Point),
    CurveTo(Point, Point, Point),
    ArcTo(Point, f32),
    Close
}
// Path of 2d geometry
pub struct Path {
    segments: Vec<PathSegment>
}
impl Path {
}