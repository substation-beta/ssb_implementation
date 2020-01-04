// Imports
use crate::g2d::vector::{
    types::Coordinate,
    point::ORIGIN_POINT,
    path::{PathBase,FlatPath,FlatPathSegment}
};
use std::{
    ops::Range,
    collections::HashMap
};


// Path to scanlines
pub fn scanlines_from_path(path: &FlatPath, area_width: u16, area_height: u16) -> Vec<(u16,Vec<Range<u16>>)> {
    scanlines_order_and_trim(
        scanlines_from_path_unordered(path, area_height),
        area_width
    )
}
#[inline]
fn scanlines_from_path_unordered(path: &FlatPath, area_height: u16) -> HashMap<u16, std::vec::Vec<f32>> {
    // Scanlines buffer
    let mut scanlines = HashMap::with_capacity(1);
    // Path to lines
    let (mut last_point, mut last_move) = (&ORIGIN_POINT, &ORIGIN_POINT);
    path.segments().iter()
    .filter_map(|segment| match segment {
        FlatPathSegment::MoveTo(point) => {
            last_move = point;
            last_point = last_move;
            None
        }
        FlatPathSegment::Close => {
            let line = if last_point != last_move {Some( (last_point, last_move) )} else {None};
            last_move = &ORIGIN_POINT;
            last_point = last_move;
            line
        }
        FlatPathSegment::LineTo(point) =>
            Some((
                {
                    let old_last_point = last_point;
                    last_point = point;
                    old_last_point
                },
                point
            ))
    })
    // Discard unwanted lines
    .filter(|line|
        line.0.y != line.1.y && // Horizontal / zero-length
        !(line.0.y < 0.0 && line.1.y < 0.0) &&  // Top-outside
        !(line.0.y >= area_height as Coordinate && line.1.y >= area_height as Coordinate)   // Bottom-outside
    )
    // Generate scanlines
    .for_each(|line| {
        // Scan range
        let (mut cur_y, last_y) = (
            (round_half_down(line.0.y.min(line.1.y)) + 0.5).max(0.5),
            (round_half_down(line.0.y.max(line.1.y)) - 0.5).min(area_height as Coordinate - 0.5)
        );
        // Straight vertical line
        if line.0.x == line.1.x {
            while cur_y <= last_y {
                scanlines.entry(cur_y.floor() as u16).or_insert_with(|| Vec::with_capacity(2) ).push(line.0.x);
                cur_y += 1.0;
            }
        }
        // Diagonal line
        else {
            let slope_x_by_y = {let line_vector = *line.1 - *line.0; line_vector.x / line_vector.y};
            while cur_y <= last_y {
                scanlines.entry(cur_y.floor() as u16).or_insert_with(|| Vec::with_capacity(2) ).push(line.0.x + (cur_y - line.0.y) * slope_x_by_y);
                cur_y += 1.0;
            }
        }
    });
    // Return unordered scanlines
    scanlines
}
#[inline]
fn scanlines_order_and_trim(mut scanlines: HashMap<u16, std::vec::Vec<f32>>, area_width: u16) -> Vec<(u16,Vec<Range<u16>>)> {
    // Sort scanlines by keys/rows
    let mut rows = scanlines.keys().copied().collect::<Vec<_>>();
    rows.sort();
    // Repack ordered scanlines
    rows.into_iter()
    .map(|row| (
        row,
        {
            // Sort scanline stops
            let mut scanline = scanlines.remove(&row).expect("Impossible! Key came from same map.");
            scanline.sort_by(|stop1, stop2| stop1.partial_cmp(stop2).expect("There isn't a not-number. Stop twitting me!") );
            // Pair & trim scanline stops
            scanline.chunks_exact(2)
            .map(|stop_pair| (
                clamp(round_half_down(stop_pair[0]), 0.0, area_width as Coordinate) as u16
                ..
                clamp(stop_pair[1].round(), 0.0, area_width as Coordinate) as u16
            ))
            // Discard empty scanline ranges
            .filter(|stop_range| !range_is_empty(&stop_range) )
            // Return optimized stops
            .collect::<Vec<_>>()
        }
    ))
    // Discard empty scanlines after stops cleaning
    .filter(|scanline| !scanline.1.is_empty() )
    // Return optimized scanlines
    .collect()
}

// Helper
#[inline]
fn round_half_down(x: Coordinate) -> Coordinate {
    if x.fract() <= 0.5 {x.floor()} else {x.ceil()}
}
#[inline]
fn clamp(x: Coordinate, min: Coordinate, max: Coordinate) -> Coordinate {   // Stabilization: <https://doc.rust-lang.org/std/primitive.f32.html#method.clamp>
    x.max(min).min(max)
}
#[inline]
fn range_is_empty(range: &Range<u16>) -> bool {   // Stabilization: <https://doc.rust-lang.org/std/ops/struct.Range.html#method.is_empty>
    range.end <= range.start
}


// Tests
#[cfg(test)]
mod tests {
    use crate::g2d::vector::{
        point::Point,
        path::{PathBase,FlatPath}
    };
    use super::scanlines_from_path;

    #[test]
    fn scanlines_quad_trimmed() {
        // Path
        let mut path = FlatPath::default();
        path.move_to(Point {x: 1.0, y: 1.0})
            .line_to(Point {x: 6.0, y: 1.0})
            .line_to(Point {x: 6.0, y: 4.0})
            .line_to(Point {x: 1.0, y: 4.0})
            .close();
        // Test
        assert_eq!(
            scanlines_from_path(&path, 5, 5),
            vec![
                (1, vec![1..5]),
                (2, vec![1..5]),
                (3, vec![1..5])
            ]
        );
    }

    #[test]
    fn scanlines_unclosed() {
        // Path
        let mut path = FlatPath::default();
        path.move_to(Point {x: 1.0, y: 0.0})
            .line_to(Point {x: 1.0, y: 3.0})
            .line_to(Point {x: 4.0, y: 3.0});
        // Test
        assert_eq!(
            scanlines_from_path(&path, 4, 3),
            vec![]
        );
    }

    #[test]
    fn scanlines_hole() {
        // Path
        let mut path = FlatPath::default();
        path.move_to(Point {x: 0.0, y: 0.0})
            .line_to(Point {x: 9.0, y: 0.0})
            .line_to(Point {x: 9.0, y: 10.0})
            .line_to(Point {x: 0.0, y: 10.0})
            .close()
            .move_to(Point {x: 2.0, y: 2.0})
            .line_to(Point {x: 2.0, y: 5.0})
            .line_to(Point {x: 7.0, y: 5.0})
            .line_to(Point {x: 7.0, y: 2.0})
            .close();
        // Test
        assert_eq!(
            scanlines_from_path(&path, 10, 10),
            vec![
                (0, vec![0..9]),
                (1, vec![0..9]),
                (2, vec![0..2, 7..9]),
                (3, vec![0..2, 7..9]),
                (4, vec![0..2, 7..9]),
                (5, vec![0..9]),
                (6, vec![0..9]),
                (7, vec![0..9]),
                (8, vec![0..9]),
                (9, vec![0..9])
            ]
        );
    }

    #[test]
    fn scanlines_subpixels() {
        // Path
        let mut path = FlatPath::default();
        path.move_to(Point {x: 1.0, y: 1.0})
            .line_to(Point {x: 4.5, y: 1.0})
            .line_to(Point {x: 6.0, y: 1.7})
            .line_to(Point {x: 8.0, y: 1.0})
            .line_to(Point {x: 9.5, y: 1.0})
            .line_to(Point {x: 9.5, y: 7.0})
            .line_to(Point {x: 2.0, y: 7.0})
            .close();
        // Test
        assert_eq!(
            scanlines_from_path(&path, 10, 10),
            vec![
                (1, vec![1..6, 7..10]),
                (2, vec![1..10]),
                (3, vec![1..10]),
                (4, vec![2..10]),
                (5, vec![2..10]),
                (6, vec![2..10])
            ]
        );
    }
}