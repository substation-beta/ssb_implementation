// Raster precision
pub type Coordinate = f32;

// Angle precision
pub type Degree = f64;

// Point of path segment
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate
}