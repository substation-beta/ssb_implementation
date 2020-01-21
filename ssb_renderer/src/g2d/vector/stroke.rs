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

// Processing
pub fn stroke_path(path: &Path, width_x: f32, width_y: f32, join: LineJoin, cap: LineCap) -> Option<Path> {

    // TODO: split to subpaths with close flags

    // TODO: prepare join & cap function (or macro?)

    // TODO: create & connect offset paths

    None
}

// Helpers
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
fn extract_sub_paths(path: &Path) -> Vec<SubPath> {
    // Out- & input
    let mut sub_paths = vec![];
    let mut segments = path.segments();
    // Extract sub paths
    let mut start_point = &ORIGIN_POINT;
    while !segments.is_empty() {
        if let Some((move_close_pos, move_close_segment)) = segments.iter().enumerate().find(|(_, segment)| match segment {
            PathSegment::Flat(FlatPathSegment::MoveTo(_)) | PathSegment::Flat(FlatPathSegment::Close) => true,
            _ => false
        }) {
            // Found subpath
            if move_close_pos > 0 {
                sub_paths.push(SubPath {
                    start: *start_point,
                    segments: SubPathSegment::from_path_segments(&segments[..move_close_pos]),
                    closed: move_close_segment == &PathSegment::Flat(FlatPathSegment::Close)
                });
            }
            start_point = if let PathSegment::Flat(FlatPathSegment::MoveTo(point)) = move_close_segment {point} else {&ORIGIN_POINT};
            segments = &segments[move_close_pos+1..];
        } else {
            // Subpath by remainings
            sub_paths.push(SubPath {
                start: *start_point,
                segments: SubPathSegment::from_path_segments(segments),
                closed: false
            });
            break;
        }
    }
    sub_paths
}


// Tests
#[cfg(test)]
mod tests {
    use super::{extract_sub_paths,PathBase,Path,Point,SubPath,SubPathSegment};

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
            extract_sub_paths(&path),
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