// Imports
use super::{
    base::{Degree, Point},
    flatten::{flatten_curve, flatten_arc}
};

// PATH BASE
trait PathBase<SegmentType> : Default + AsRef<[SegmentType]> {
    fn new(segments: Vec<SegmentType>) -> Self;

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
#[derive(Debug, Clone, Default)]
pub struct FlatPath {
    segments: Vec<FlatPathSegment>
}
impl AsRef<[FlatPathSegment]> for FlatPath {
    fn as_ref(&self) -> &[FlatPathSegment] {
        &self.segments
    }
}
impl PathBase<FlatPathSegment> for FlatPath {
    fn new(segments: Vec<FlatPathSegment>) -> Self {
        Self {
            segments
        }
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
        // Result buffer
        let mut flat_segments = Vec::with_capacity(path.as_ref().len());
        // Flatten path segments
        path.segments.into_iter().fold(None, |last_point, segment| {
            match segment {
                // Repack already flat segments
                PathSegment::Flat(flat_segment) => {
                    let new_last_point = match flat_segment {
                        FlatPathSegment::MoveTo(point) => Some(point),
                        FlatPathSegment::LineTo(point) => Some(point),
                        FlatPathSegment::Close => last_point
                    };
                    flat_segments.push(flat_segment);
                    new_last_point
                }
                // Flatten curve
                PathSegment::CurveTo(control_point1, control_point2, end_point) => {
                    let curve_points = flatten_curve(last_point.unwrap_or_default(), control_point1, control_point2, end_point);

                    // TODO: points to lines
                    unimplemented!();

                    Some(end_point)
                }
                // Flatten arc
                PathSegment::ArcBy(center_point, angle) => {
                    let arc_points = flatten_arc(last_point.unwrap_or_default(), center_point, angle);

                    // TODO: points to lines
                    unimplemented!();

                    arc_points.last().map_or(last_point, |point| Some(*point))
                }
            }
        });
        // Create flat path
        Self::new(flat_segments)
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
#[derive(Debug, Clone, Default)]
pub struct Path {
    segments: Vec<PathSegment>
}
impl AsRef<[PathSegment]> for Path {
    fn as_ref(&self) -> &[PathSegment] {
        &self.segments
    }
}
impl PathBase<PathSegment> for Path {
    fn new(segments: Vec<PathSegment>) -> Self {
        Self {
            segments
        }
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
        Self::new(
            flat_path.segments.into_iter().map(|flat_segment| PathSegment::Flat(flat_segment)).collect()
        )
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