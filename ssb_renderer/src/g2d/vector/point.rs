// Imports
use super::types::Coordinate;
use crate::g2d::math::FloatExt;
use std::ops::{Add,AddAssign,Sub,Mul};


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
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
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
#[allow(clippy::len_without_is_empty)]
impl Point {
    pub fn len(self) -> Coordinate {
        self.x.hypot(self.y)
    }
    pub fn grid_len(self) -> Coordinate {
        self.x.abs() + self.y.abs()
    }
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round()
        }
    }
    pub fn round_half_down(self) -> Self {
        Self {
            x: self.x.round_half_down(),
            y: self.y.round_half_down()
        }
    }
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y)
        }
    }
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y)
        }
    }
}

// Point collections
pub trait PointMinMaxCollector<'origin>: Iterator<Item=&'origin Point> {
    fn min_max(self) -> Option<(Point,Point)>;
}
impl <'origin, I: Iterator<Item=&'origin Point>> PointMinMaxCollector<'origin> for I {
    fn min_max(self) -> Option<(Point,Point)> {
        self.fold(None, |mut min_max_points, point|
            if let Some((min_point, max_point)) = min_max_points.as_mut() {
                if point.x < min_point.x {min_point.x = point.x;}
                if point.y < min_point.y {min_point.y = point.y;}
                if point.x > max_point.x {max_point.x = point.x;}
                if point.y > max_point.y {max_point.y = point.y;}
                min_max_points
            } else {
                Some((
                    *point,
                    *point
                ))
            }
        )
    }
}

// Default point (possible to reference)
pub static ORIGIN_POINT: Point = Point {x: 0.0, y: 0.0};