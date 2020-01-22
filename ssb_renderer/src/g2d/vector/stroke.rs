// Imports
use super::{
    path::{PathBase,Path,PathSegment,FlatPathSegment},
    point::{Point,ORIGIN_POINT}
};


// Shape of line endings
pub enum LineJoin {
    ROUND,
    BEVEL,
    MITER(f32)  // Limit
}
pub enum LineCap {
    ROUND,
    BUTT,
    SQUARE
}

// Stroke processing
pub fn stroke_path(path: &Path, width_x: f32, width_y: f32, join: LineJoin, cap: LineCap) -> Path {
    let stroke_path_segments = vec![];
    for sub_path in SubPathIterator::new(path.segments()) {

        // TODO: prepare join & cap function (or macro?)

        // TODO: create & connect offset paths

    }
    Path::new(stroke_path_segments)
}

// Sub paths
#[derive(Debug,PartialEq)]
enum SubPathSegment {
    LineTo(Point),
    CurveTo(Point, Point, Point)
}
impl SubPathSegment {
    pub fn from_path_segments(segments: &[PathSegment]) -> Vec<Self> {
        segments.iter().filter_map(|segment| match segment {
            PathSegment::Flat(FlatPathSegment::LineTo(point)) => Some(SubPathSegment::LineTo(*point)),
            PathSegment::CurveTo(control_point1, control_point2, end_point) => Some(SubPathSegment::CurveTo(*control_point1, *control_point2, *end_point)),
            _ => None
        }).collect()
    }
}
#[derive(Debug,PartialEq)]
struct SubPath {
    start: Point,
    segments: Vec<SubPathSegment>,
    closed: bool
}
struct SubPathIterator<'origin> {
    segments: &'origin [PathSegment],
    start_point: &'origin Point
}
impl<'origin> SubPathIterator<'origin> {
    pub fn new(segments: &'origin [PathSegment]) -> Self {
        Self {
            segments,
            start_point: &ORIGIN_POINT
        }
    }
}
impl<'origin> Iterator for SubPathIterator<'origin> {
    type Item = SubPath;
    fn next(&mut self) -> Option<Self::Item> {
        // More path segments to process?
        while !self.segments.is_empty() {
            if let Some((move_close_pos, move_close_segment)) = self.segments.iter().enumerate().find(|(_, segment)| match segment {
                PathSegment::Flat(FlatPathSegment::MoveTo(_)) | PathSegment::Flat(FlatPathSegment::Close) => true,
                _ => false
            }) {
                // Found subpath
                let sub_path = if move_close_pos > 0 {
                    Some(SubPath {
                        start: *self.start_point,
                        segments: SubPathSegment::from_path_segments(&self.segments[..move_close_pos]),
                        closed: move_close_segment == &PathSegment::Flat(FlatPathSegment::Close)
                    })
                } else {
                    None
                };
                self.start_point = if let PathSegment::Flat(FlatPathSegment::MoveTo(point)) = move_close_segment {point} else {&ORIGIN_POINT};
                self.segments = &self.segments[move_close_pos+1..];
                if sub_path.is_some() {
                    return sub_path;
                }
            } else {
                // Subpath by remainings
                let sub_path = Some(SubPath {
                    start: *self.start_point,
                    segments: SubPathSegment::from_path_segments(self.segments),
                    closed: false
                });
                self.segments = &self.segments[0..0];
                return sub_path;
            }
        }
        None
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::{SubPathIterator,PathBase,Path,Point,SubPath,SubPathSegment};

    #[test]
    fn sub_paths() {
        // Path
        let mut path = Path::default();
        path.curve_to(Point {x: 10.0, y: 5.0}, Point {x: 20.0, y: -8.0}, Point {x: 40.0, y: 0.0})
            .line_to(Point {x: 20.0, y: 55.0})
            .close()
            .move_to(Point {x: -42.0, y: 9.0})
            .line_to(Point {x: 42.0, y: 9.0})
            .line_to(Point {x: 0.0, y: -9999.0});
        // Test
        assert_eq!(
            SubPathIterator::new(path.segments()).collect::<Vec<_>>(),
            vec![
                SubPath {
                    start: Point {x: 0.0, y: 0.0},
                    segments: vec![
                        SubPathSegment::CurveTo(Point {x: 10.0, y: 5.0}, Point {x: 20.0, y: -8.0}, Point {x: 40.0, y: 0.0}),
                        SubPathSegment::LineTo(Point {x: 20.0, y: 55.0})
                    ],
                    closed: true
                },
                SubPath {
                    start: Point {x: -42.0, y: 9.0},
                    segments: vec![
                        SubPathSegment::LineTo(Point {x: 42.0, y: 9.0}),
                        SubPathSegment::LineTo(Point {x: 0.0, y: -9999.0})
                    ],
                    closed: false
                }
            ]
        );
    }
}