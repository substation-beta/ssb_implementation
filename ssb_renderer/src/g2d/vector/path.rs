// Imports
use super::{
    types::{Coordinate,Degree},
    point::{Point,ORIGIN_POINT},
    flatten::{flatten_curve, flatten_arc}
};


// PATH BASE
pub trait PathBase<SegmentType> : Default {
    fn new(segments: Vec<SegmentType>) -> Self;
    fn segments(&self) -> &[SegmentType];

    fn move_to(&mut self, point: Point) -> &mut Self;
    fn line_to(&mut self, point: Point) -> &mut Self;
    fn close(&mut self) -> &mut Self;
}

// FLAT PATH
// Segment of flat path
#[derive(Debug, Clone, PartialEq)]
pub enum FlatPathSegment {
    MoveTo(Point),
    LineTo(Point),
    Close
}

// Flat path of 2d geometry
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlatPath {
    segments: Vec<FlatPathSegment>
}
impl PathBase<FlatPathSegment> for FlatPath {
    fn new(segments: Vec<FlatPathSegment>) -> Self {
        Self {
            segments
        }
    }
    fn segments(&self) -> &[FlatPathSegment] {
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
        // Result buffer
        let mut flat_segments = Vec::with_capacity(path.segments.len());
        // Flatten path segments
        path.segments.into_iter().fold(ORIGIN_POINT, |last_point, segment| {
            match segment {
                // Repack already flat segments
                PathSegment::Flat(flat_segment) => {
                    let new_last_point = match flat_segment {
                        FlatPathSegment::MoveTo(point) | FlatPathSegment::LineTo(point) => point,
                        FlatPathSegment::Close => last_point
                    };
                    flat_segments.push(flat_segment);
                    new_last_point
                }
                // Flatten curve
                PathSegment::CurveTo(control_point1, control_point2, end_point) => {
                    flatten_curve(last_point, control_point1, control_point2, end_point).into_iter()
                        .skip(1)
                        .inspect(|point| flat_segments.push(FlatPathSegment::LineTo(*point)) )
                        .last().unwrap_or(last_point)
                }
                // Flatten arc
                PathSegment::ArcBy(center_point, angle) => {
                    flatten_arc(last_point, center_point, angle).into_iter()
                        .skip(1)
                        .inspect(|point| flat_segments.push(FlatPathSegment::LineTo(*point)) )
                        .last().unwrap_or(last_point)
                }
            }
        });
        // Create flat path
        Self::new(flat_segments)
    }
}
impl FlatPath {
    pub fn bounding(&self) -> Option<(Point, Point)> {
        self.segments.iter()
        .filter_map(|segment| match segment {
            FlatPathSegment::MoveTo(point) | FlatPathSegment::LineTo(point) => Some(point),
            FlatPathSegment::Close => None
        })
        .fold(None, |mut min_max_points, point|
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
    pub fn translate(&mut self, x: Coordinate, y: Coordinate) -> &mut Self {
        let translate_point = Point {x, y};
        self.segments.iter_mut()
        .filter_map(|segment| match segment {
            FlatPathSegment::MoveTo(point) | FlatPathSegment::LineTo(point) => Some(point),
            FlatPathSegment::Close => None
        })
        .for_each(|point| *point += translate_point );
        self
    }
}

// PATH
// Segment of path
#[derive(Debug, Clone, PartialEq)]
pub enum PathSegment {
    Flat(FlatPathSegment),
    CurveTo(Point, Point, Point),   // Control points + end point
    ArcBy(Point, Degree),  // Orientation/center point + angle
}

// Path of 2d geometry
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Path {
    segments: Vec<PathSegment>
}
impl PathBase<PathSegment> for Path {
    fn new(segments: Vec<PathSegment>) -> Self {
        Self {
            segments
        }
    }
    fn segments(&self) -> &[PathSegment] {
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
        Self::new(
            flat_path.segments.into_iter().map(PathSegment::Flat).collect()
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


// Tests
#[cfg(test)]
mod tests {
    use super::{PathBase,Point,Path,FlatPath,PathSegment,FlatPathSegment};

    fn create_path() -> Path {
        let mut path = Path::default();
        path.move_to(Point {x: 0.0, y: 50.0})
            .line_to(Point {x: 0.0, y: 0.0})
            .arc_by(Point {x: 0.0, y: 50.0}, 180.0)
            .curve_to(Point {x: 35.0, y: 90.0}, Point {x: -75.0, y: 60.0}, Point {x: 0.0, y: 50.0})
            .close();
        path
    }

    #[test]
    fn path_build() {
        assert_eq!(
            create_path().segments(),
            &[
                PathSegment::Flat(FlatPathSegment::MoveTo(Point {x: 0.0, y: 50.0})),
                PathSegment::Flat(FlatPathSegment::LineTo(Point {x: 0.0, y: 0.0})),
                PathSegment::ArcBy(Point {x: 0.0, y: 50.0}, 180.0),
                PathSegment::CurveTo(Point {x: 35.0, y: 90.0}, Point {x: -75.0, y: 60.0}, Point {x: 0.0, y: 50.0}),
                PathSegment::Flat(FlatPathSegment::Close)
            ]
        );
    }

    #[test]
    fn path_flatten() {
        let flat_path = FlatPath::from(create_path());
        let flat_path_segments = flat_path.segments();
        assert_eq!(flat_path_segments.first(), Some(&FlatPathSegment::MoveTo(Point {x: 0.0, y: 50.0})));
        assert_eq!(flat_path_segments.get(1), Some(&FlatPathSegment::LineTo(Point {x: 0.0, y: 0.0})));
        assert_eq!(flat_path_segments.last(), Some(&FlatPathSegment::Close));
        assert!(
            flat_path_segments.into_iter()
                .skip(2)
                .take(flat_path_segments.len()-3)
                .all(|segment| match segment {
                    FlatPathSegment::LineTo(point) if (-70.0..=50.0).contains(&point.x) && (0.0..=100.0).contains(&point.y) => true,
                    _ => false
                }),
            "Flattening curves & arcs failed: {:?}", flat_path
        );
    }

    fn create_flat_path() -> FlatPath {
        let mut flat_path = FlatPath::default();
        flat_path.move_to(Point {x: 5.0, y: 5.0})
            .line_to(Point {x: 0.0, y: 1.0})
            .line_to(Point {x: 99.0, y: 0.2})
            .close()
            .move_to(Point {x: 7.0, y: 33.0})
            .line_to(Point {x: 2.0, y: 25.0})
            .line_to(Point {x: 7.0, y: 31.0})
            .close();
        flat_path
    }

    #[test]
    fn path_bounding() {
        assert_eq!(
            create_flat_path().bounding(),
            Some((
                Point {x: 0.0, y: 0.2},
                Point {x: 99.0, y: 33.0}
            ))
        );
    }

    #[test]
    fn path_translate() {
        let mut flat_path = create_flat_path();
        flat_path.translate(-22.5, 1.0);
        assert_eq!(
            flat_path.segments(),
            &[
                FlatPathSegment::MoveTo(Point {x: -17.5, y: 6.0}),
                FlatPathSegment::LineTo(Point {x: -22.5, y: 2.0}),
                FlatPathSegment::LineTo(Point {x: 76.5, y: 1.2}),
                FlatPathSegment::Close,
                FlatPathSegment::MoveTo(Point {x: -15.5, y: 34.0}),
                FlatPathSegment::LineTo(Point {x: -20.5, y: 26.0}),
                FlatPathSegment::LineTo(Point {x: -15.5, y: 32.0}),
                FlatPathSegment::Close
            ]
        );
    }
}