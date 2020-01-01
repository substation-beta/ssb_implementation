// Imports
use super::types::Coordinate;
use std::ops::{Add,Sub,Mul};


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
impl Mul<Coordinate> for Point {
    type Output = Self;
    fn mul(self, factor: Coordinate) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor
        }
    }
}
impl Point {
    pub fn len(&self) -> Coordinate {
        self.x.hypot(self.y)
    }
}