// Imports
use super::{
    types::{Coordinate,Degree},
    point::{Point,GenericPoint}
};


// Split arc into cubic curves
pub fn arc_to_curves(start_point: Point, center_point: Point, angle: Degree) -> Vec<[Point;4]> {
    // Anything to do?
    if angle != 0.0 {
        // Curves buffer
        let (full_curves_n, last_curve_angle) = (
            (angle / 90.0).abs().floor() as u16,
            angle % 90.0
        );
        let mut curves = Vec::with_capacity(if last_curve_angle != 0.0 {full_curves_n + 1} else {full_curves_n} as usize);
        // Magic numbers
        const CONTROL_POINT_DISTANCE: Degree = 0.390_262_856_458_446_8; // 0.551915024494 / 1_f64.hypot(1.) | Perfect radial distance relative to start-to-end vector
        const SIN_COS_45: Coordinate = std::f64::consts::FRAC_1_SQRT_2 as Coordinate;
        // Generate full curves
        let mut vector = start_point - center_point;
        let angle_direction = angle.signum() as Coordinate;
        for _ in 0..full_curves_n {
            // Calculate end point
            let rotated_vector = Point {
                x: -vector.y * angle_direction,
                y: vector.x * angle_direction
            };
            // Calculate vectors to control points
            let start_end_line = (rotated_vector - vector) * CONTROL_POINT_DISTANCE as Coordinate * SIN_COS_45;
            let end_start_line = -start_end_line;
            // Save curve
            curves.push([
                center_point + vector,
                center_point + vector + Point {
                    x: start_end_line.x + start_end_line.y * angle_direction,
                    y: start_end_line.y - start_end_line.x * angle_direction
                },
                center_point + rotated_vector + Point {
                    x: end_start_line.x - end_start_line.y * angle_direction,
                    y: end_start_line.x * angle_direction + end_start_line.y
                },
                center_point + rotated_vector
            ]);
            vector = rotated_vector;
        }
        // Generate last curve
        if last_curve_angle != 0.0 {
            // Calculate end point
            let precise_vector = GenericPoint::<Degree>::from(vector);
            let last_curve_rad = last_curve_angle.to_radians();
            let (angle_sin, angle_cos) = (last_curve_rad.sin(), last_curve_rad.cos());
            let precise_rotated_vector = GenericPoint {
                x: precise_vector.x * angle_cos - precise_vector.y * angle_sin,
                y: precise_vector.y * angle_cos + precise_vector.x * angle_sin
            };
            // Calculate vectors to control points
            let start_end_line = (precise_rotated_vector - precise_vector) * CONTROL_POINT_DISTANCE;
            let end_start_line = -start_end_line;
            // Save curve
            let last_curve_half_rad = last_curve_rad * 0.5;
            let (half_angle_sin, half_angle_cos) = (last_curve_half_rad.sin(), last_curve_half_rad.cos());
            curves.push([
                center_point + vector,
                center_point + vector + Point {
                    x: (start_end_line.x * half_angle_cos + start_end_line.y * half_angle_sin) as Coordinate,
                    y: (start_end_line.y * half_angle_cos - start_end_line.x * half_angle_sin) as Coordinate
                },
                center_point + Point::from(precise_rotated_vector) + Point {
                    x: (end_start_line.x * half_angle_cos - end_start_line.y * half_angle_sin) as Coordinate,
                    y: (end_start_line.y * half_angle_cos + end_start_line.x * half_angle_sin) as Coordinate
                },
                center_point + Point::from(precise_rotated_vector)
            ]);
        }
        // Return collected curves
        curves
    } else {
        vec![]
    }
}

// Flatten curve to polyline
const CURVE_DEVIATION_LENGTH: Coordinate = 0.125;
#[inline]
fn is_curve_flat(p: &[Point;4]) -> bool {
    (p[1] - p[0]).len() + (p[2] - p[1]).len() + (p[3] - p[2]).len()
    <
    (p[3] - p[0]).len() + CURVE_DEVIATION_LENGTH
}
#[inline]
fn split_curve_mid(p: [Point;4]) -> ([Point;4], [Point;4]) {
    // Calculate intermediate points
    const T: Coordinate = 0.5;
    let (p01, p12, p23) = (
        p[0] + (p[1] - p[0]) * T,
        p[1] + (p[2] - p[1]) * T,
        p[2] + (p[3] - p[2]) * T
    );
    let (p012, p123) = (
        p01 + (p12 - p01) * T,
        p12 + (p23 - p12) * T
    );
    let p1234 = p012 + (p123 - p012) * T;
    // Assemble both curves
    (
        [p[0], p01, p012, p1234],
        [p1234, p123, p23, p[3]]
    )
}
pub fn flatten_curve(start_point: Point, control_point1: Point, control_point2: Point, end_point: Point) -> Vec<Point> {
    let mut points = vec![start_point];
    let mut curves = vec![[start_point, control_point1, control_point2, end_point]];
    while let Some(curve) = curves.pop() {
        // Flat enough = line
        if is_curve_flat(&curve) {
            points.push(curve[3]);
        } else {
            // Try again with subdivided curve
            let (curve1, curve2) = split_curve_mid(curve);
            curves.push(curve2);
            curves.push(curve1);
        }
    }
    points
}


// Tests
#[cfg(test)]
mod tests {
    use super::{Point,arc_to_curves,flatten_curve};

    #[test]
    fn arc_curves() {
        // Short positive-angled arc
        let curve = arc_to_curves(Point {x: 10.0, y: 10.0}, Point {x: 10.0, y: 50.0}, 45.0);
        assert!(
            {
                let end_point = curve.first().expect("One curve must exist for this input!")[3];
                (38.0..38.5).contains(&end_point.x) && (21.5..22.0).contains(&end_point.y)
            },
            "Short curve points unexpected: {:?}", curve
        );
        // Long negative-angled arc
        assert_eq!(
            arc_to_curves(Point {x: 0.0, y: 0.0}, Point {x: 0.0, y: 100.0}, -450.0),
            vec![
                [Point {x: 0.0, y: 0.0}, Point {x: -55.191498, y: 0.0}, Point {x: -100.0, y: 44.808502}, Point {x: -100.0, y: 100.0}],
                [Point {x: -100.0, y: 100.0}, Point {x: -100.0, y: 155.1915}, Point {x: -55.191498, y: 200.0}, Point {x: 0.0, y: 200.0}],
                [Point {x: 0.0, y: 200.0}, Point {x: 55.191498, y: 200.0}, Point {x: 100.0, y: 155.1915}, Point {x: 100.0, y: 100.0}],
                [Point {x: 100.0, y: 100.0}, Point {x: 100.0, y: 44.808502}, Point {x: 55.191498, y: 0.0}, Point {x: 0.0, y: 0.0}],
                [Point {x: 0.0, y: 0.0}, Point {x: -55.191498, y: 0.0}, Point {x: -100.0, y: 44.808502}, Point {x: -100.0, y: 100.0}]
            ]
        );
    }

    #[test]
    fn flat_curve() {
        // Already flat/line
        assert_eq!(
            flatten_curve(Point {x: -2.0, y: 7.0}, Point {x: -1.0, y: 7.0}, Point {x: 0.0, y: 7.0}, Point {x: 1.0, y: 7.0}),
            vec![Point {x: -2.0, y: 7.0}, Point {x: 1.0, y: 7.0}]
        );
        // Complex
        let points = flatten_curve(Point {x: -5.0, y: 0.0}, Point {x: 0.0, y: -4.0}, Point {x: 5.0, y: 6.0}, Point {x: 10.0, y: 1.0});
        assert!(points.len() > 5, "Not enough points: {:?}", points);
    }
}