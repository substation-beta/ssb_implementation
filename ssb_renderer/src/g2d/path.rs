// Imports
use std::convert::From;
use super::base::{Degree, Point};

// PATH BASE
trait PathBase<SegmentType> {
    fn new() -> Self;
    fn new_by_segments(segments: Vec<SegmentType>) -> Self;

    fn get_segments(&self) -> &Vec<SegmentType>;

    fn move_to(&mut self, point: Point) -> &mut Self;
    fn line_to(&mut self, point: Point) -> &mut Self;
    fn close(&mut self) -> &mut Self;
}

// FLAT PATH
// Segment of flat path
#[derive(Debug, Clone)]
pub enum FlatPathSegment {
    MoveTo(Point),
    LineTo(Point),
    Close
}

// Flat path of 2d geometry
#[derive(Debug, Clone)]
pub struct FlatPath {
    segments: Vec<FlatPathSegment>
}
impl PathBase<FlatPathSegment> for FlatPath {
    fn new() -> Self {
        Self {
            segments: vec!()
        }
    }
    fn new_by_segments(segments: Vec<FlatPathSegment>) -> Self {
        Self {
            segments
        }
    }

    fn get_segments(&self) -> &Vec<FlatPathSegment> {
        &self.segments
    }

    fn move_to(&mut self, point: Point) -> &mut Self {
        self.segments.push(FlatPathSegment::MoveTo(point));
        self
    }
    fn line_to(&mut self, point: Point) -> &mut Self {
        self.segments.push(FlatPathSegment::LineTo(point));
        self
    }
    fn close(&mut self) -> &mut Self {
        self.segments.push(FlatPathSegment::Close);
        self
    }
}
impl From<Path> for FlatPath {
    fn from(path: Path) -> Self {
        let mut flat_segments = Vec::with_capacity(path.get_segments().len());
        path.segments.into_iter().for_each(|segment| {
            match segment {
                PathSegment::Flat(flat_segment) => flat_segments.push(flat_segment),
                PathSegment::CurveTo(control_point1, control_point2, end_point) => unimplemented!(),
                PathSegment::ArcBy(center_point, angle) => unimplemented!()
            }
        });
        Self::new_by_segments(flat_segments)
    }
}

// PATH
// Segment of path
#[derive(Debug, Clone)]
pub enum PathSegment {
    Flat(FlatPathSegment),
    CurveTo(Point, Point, Point),   // Control points + end point
    ArcBy(Point, Degree),  // Orientation/center point + angle
}
// Path of 2d geometry
#[derive(Debug, Clone)]
pub struct Path {
    segments: Vec<PathSegment>
}
impl PathBase<PathSegment> for Path {
    fn new() -> Self {
        Self {
            segments: vec!()
        }
    }
    fn new_by_segments(segments: Vec<PathSegment>) -> Self {
        Self {
            segments
        }
    }

    fn get_segments(&self) -> &Vec<PathSegment> {
        &self.segments
    }

    fn move_to(&mut self, point: Point) -> &mut Self {
        self.segments.push(PathSegment::Flat(FlatPathSegment::MoveTo(point)));
        self
    }
    fn line_to(&mut self, point: Point) -> &mut Self {
        self.segments.push(PathSegment::Flat(FlatPathSegment::LineTo(point)));
        self
    }
    fn close(&mut self) -> &mut Self {
        self.segments.push(PathSegment::Flat(FlatPathSegment::Close));
        self
    }
}
impl From<FlatPath> for Path {
    fn from(flat_path: FlatPath) -> Self {
        Self::new_by_segments(flat_path.segments.into_iter().map(|flat_segment| PathSegment::Flat(flat_segment)).collect())
    }
}
impl Path {
    pub fn curve_to(&mut self, control_point1: Point, control_point2: Point, end_point: Point) -> &mut Self {
        self.segments.push(PathSegment::CurveTo(control_point1, control_point2, end_point));
        self
    }
    pub fn arc_by(&mut self, center_point: Point, angle: Degree) -> &mut Self {
        self.segments.push(PathSegment::ArcBy(center_point, angle));
        self
    }
}

// TESTS
#[cfg(test)]
mod tests {

    // TODO: test all cases
    
}