// Imports
use super::{
    types::Degree,
    point::Point
};


// Flatten curve to polyline
pub fn flatten_curve(start_point: Point, control_point1: Point, control_point2: Point, end_point: Point) -> Vec<Point> {

    // TODO: flatten by fast-precise algorithm with tolerance
    unimplemented!()

}

// Flatten arc to polyline
pub fn flatten_arc(start_point: Point, center_point: Point, angle: Degree) -> Vec<Point> {

    // TODO: flatten by tolerance
    unimplemented!()

}

// Tests
#[cfg(test)]
mod tests {
    use super::{flatten_curve, flatten_arc};

    #[test]
    fn flat_curve() {

        // TODO

    }

    #[test]
    fn flat_arc() {

        // TODO

    }
}