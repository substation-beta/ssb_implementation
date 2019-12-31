// Imports
use super::types::Coordinate;
use std::ops::{Add,Sub};


// Point of path segment
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate
}

// Point math
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
impl Point {
    pub fn len(&self) -> Coordinate {
        self.x.hypot(self.y)
    }
}