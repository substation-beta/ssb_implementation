// Raster precision
pub type Coordinate = f32;

// Angle precision
pub type Degree = f32;

// Point of path segment
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: Coordinate,
    y: Coordinate
}