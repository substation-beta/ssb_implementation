// Raster precision
pub type Coordinate = f32;

// Angle precision
pub type Degree = f32;

// Point of path segment
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate
}