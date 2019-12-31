// Imports
use super::{
    types::{Coordinate,Degree},
    point::Point
};


// Flatten curve to polyline
pub fn flatten_curve(start_point: Point, control_point1: Point, control_point2: Point, end_point: Point) -> Vec<Point> {

    // TODO: flatten by fast-precise algorithm with tolerance
    unimplemented!()

}

// Flatten arc to polyline
const ARC_LINE_LENGTH: Degree = 0.75;
pub fn flatten_arc(start_point: Point, center_point: Point, angle: Degree) -> Vec<Point> {
    // Anything to do?
    if start_point == center_point || angle == 0.0 {
        return vec![start_point];
    }
    // Vector between points
    let vector = start_point - center_point;
    // Number of required lines
    let angle_rad = angle.to_radians();
    let lines_n = angle_rad.abs() * vector.len() as Degree / ARC_LINE_LENGTH;
    let lines_n_ceil = lines_n.ceil();
    // Points buffer
    let mut points = Vec::with_capacity(1 + lines_n_ceil as usize);
    // Add start point
    points.push(start_point);
    // Add intermediate points
    if lines_n >= 1.0 {
        let mut vector_part = vector;
        let angle_rad_part = angle_rad / lines_n_ceil;
        let angle_sin_part = angle_rad_part.sin();
        let angle_cos_part = angle_rad_part.cos();
        for _ in 1..lines_n_ceil as usize {
            points.push(center_point + {
                vector_part = Point {
                    x: (vector_part.x as Degree * angle_cos_part - vector_part.y as Degree * angle_sin_part) as Coordinate,
                    y: (vector_part.x as Degree * angle_sin_part + vector_part.y as Degree * angle_cos_part) as Coordinate
                };
                vector_part
            });
        }
    }
    // Add end point
    if lines_n_ceil > lines_n {
        let angle_sin = angle_rad.sin();
        let angle_cos = angle_rad.cos();
        points.push(center_point + Point {
            x: (vector.x as Degree * angle_cos - vector.y as Degree * angle_sin) as Coordinate,
            y: (vector.x as Degree * angle_sin + vector.y as Degree * angle_cos) as Coordinate
        });
    }
    // Return points
    points
}

// Tests
#[cfg(test)]
mod tests {
    use super::{Point, flatten_curve, flatten_arc};

    #[test]
    fn flat_curve() {

        // TODO

    }

    #[test]
    fn flat_arc() {
        // Nothing to do
        assert_eq!(
            flatten_arc(Point {x: 42.5, y: 33.0}, Point {x: 42.5, y: 33.0}, 99.0),
            vec![Point {x: 42.5, y: 33.0}]
        );
        assert_eq!(
            flatten_arc(Point {x: 1.0, y: 2.0}, Point {x: 3.0, y: 4.0}, 0.0),
            vec![Point {x: 1.0, y: 2.0}]
        );
        // Tiny angle = 2 points = 1 line
        assert_eq!(
            flatten_arc(Point {x: 0.0, y: -5.0}, Point {x: 0.0, y: 0.0}, 0.00001).len(),
            2
        );
        // Complex
        let points = flatten_arc(Point {x: 0.0, y: -4.0}, Point {x: 0.0, y: -2.0}, 90.0);
        assert_eq!(points.last(), Some(&Point{x: 2.0, y: -2.0}), "Points: {:?}", points);
    }
}