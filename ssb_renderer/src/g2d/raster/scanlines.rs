// Imports
use crate::g2d::{
    math::{FloatExt,RangeExt},
    vector::{
        types::Coordinate,
        point::ORIGIN_POINT,
        path::{PathBase,FlatPath,FlatPathSegment}
    }
};
use std::{
    mem::replace as memory_replace,
    ops::Range,
    collections::{HashMap,BTreeMap}
};


// Path to scanlines
pub fn scanlines_from_path(path: &FlatPath, area_width: u16, area_height: u16) -> HashMap<u16,Vec<Range<u16>>> {
    // Scanlines buffer
    let mut scanlines = HashMap::with_capacity(1);
    struct ScanlineStop {
        x: Coordinate,
        winding: i8
    }
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
                memory_replace(&mut last_point, point),
                point
            ))
    })
    // Discard unwanted lines
    .filter(|line|
        !line.0.y.eq_close(line.1.y)  && // Horizontal / zero-length
        !(line.0.y < 0.0 && line.1.y < 0.0) &&  // Top-outside
        !(line.0.y >= area_height as Coordinate && line.1.y >= area_height as Coordinate)   // Bottom-outside
    )
    // Generate scanlines
    .for_each(|line| {
        // Scan range
        let (mut cur_y, last_y) = (
            (line.0.y.min(line.1.y).round_half_down() + 0.5).max(0.5),
            (line.0.y.max(line.1.y).round_half_down() - 0.5).min(area_height as Coordinate - 0.5)
        );
        // Line direction as winding rule
        let winding = (line.1.y - line.0.y).signum() as i8;
        // Straight vertical line
        if line.0.x.eq_close(line.1.x) {
            while cur_y <= last_y {
                scanlines.entry(cur_y.floor() as u16).or_insert_with(|| Vec::with_capacity(2) ).push(ScanlineStop {
                    x: line.0.x,
                    winding
                });
                cur_y += 1.0;
            }
        }
        // Diagonal line
        else {
            let slope_x_by_y = {let line_vector = *line.1 - *line.0; line_vector.x / line_vector.y};
            while cur_y <= last_y {
                scanlines.entry(cur_y.floor() as u16).or_insert_with(|| Vec::with_capacity(2) ).push(ScanlineStop {
                    x: line.0.x + (cur_y - line.0.y) * slope_x_by_y,
                    winding
                });
                cur_y += 1.0;
            }
        }
    });
    // Convert scanline stops to ranges
    scanlines.into_iter()
    .map(|(row,mut scanline)| (
        row,
        {
            // Sort scanline stops
            scanline.sort_by(|stop1, stop2| stop1.x.partial_cmp(&stop2.x).expect("There isn't a NAN. Stop twitting me!") );
            // Pair & trim scanline stops to ranges
            let (mut coverage, mut range_start_option) = (0, None as Option<Coordinate>);
            scanline.into_iter()
            .filter_map(|stop| {
                let range = match range_start_option {
                    // Complete range
                    Some(range_start) if coverage + stop.winding as i16 == 0 => {
                        range_start_option = None;
                        Some(
                            FloatExt::clamp(range_start.round_half_down(), 0.0, area_width as Coordinate) as u16
                            ..
                            FloatExt::clamp(stop.x.round(), 0.0, area_width as Coordinate) as u16
                        )
                    }
                    // Save range start
                    None if coverage == 0 => {
                        range_start_option = Some(stop.x);
                        None
                    }
                    // Ignore over-coverage
                    _ => None
                };
                coverage += stop.winding as i16;
                range
            })
            // Discard empty scanline ranges
            .filter(|stop_range| !RangeExt::is_empty(stop_range) )
            // Return ranges
            .collect::<Vec<_>>()
        }
    ))
    // Discard empty scanlines after stops cleaning
    .filter(|(_,scanline)| !scanline.is_empty() )
    // Return optimized scanlines
    .collect()
}

// Merge scanlines from different samples and sort everything
pub fn merge_and_order_scanlines(mut scanlines_samples: Vec<HashMap<u16,Vec<Range<u16>>>>) -> BTreeMap<u16,Vec<Range<u16>>> {
    // Anything to do?
    if let Some((scanlines, other_scanlines_samples)) = scanlines_samples.split_first_mut() {
        // Merge all scanlines into first sample
        for (row, other_scanline_ranges) in other_scanlines_samples.iter_mut().flatten() {
            if let Some(scanline_ranges) = scanlines.get_mut(row) {
                scanline_ranges.append(other_scanline_ranges);
            } else {
                scanlines.insert(*row, other_scanline_ranges.drain(..).collect());
            }
        }
        // Convert scanlines to sorted map with sorted ranges
        scanlines.drain()
        .map(|mut scanline| {
            scanline.1.sort_by(|range1, range2| range1.start.cmp(&range2.start) );
            scanline
        })
        .collect::<BTreeMap<_,_>>()
    } else {
        BTreeMap::new()
    }
}


// Tests
#[cfg(test)]
mod tests {
    use crate::g2d::vector::{
        point::Point,
        path::{PathBase,FlatPath}
    };
    use super::{scanlines_from_path,HashMap,merge_and_order_scanlines};

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
            HashMap::new()
        );
    }

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
            [
                (1, vec![1..5]),
                (2, vec![1..5]),
                (3, vec![1..5])
            ].iter().cloned().collect()    // To map for comparison
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
            [
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
            ].iter().cloned().collect()    // To map for comparison
        );
    }

    #[test]
    fn scanlines_winding() {
        // Path
        let mut path = FlatPath::default();
        path.move_to(Point {x: 0.0, y: 0.0})
            .line_to(Point {x: 9.0, y: 0.0})
            .line_to(Point {x: 9.0, y: 10.0})
            .line_to(Point {x: 0.0, y: 10.0})
            .close()
            .move_to(Point {x: 2.0, y: 2.0})
            .line_to(Point {x: 7.0, y: 2.0})
            .line_to(Point {x: 7.0, y: 5.0})
            .line_to(Point {x: 2.0, y: 5.0})
            .close();
        // Test
        assert_eq!(
            scanlines_from_path(&path, 10, 10),
            [
                (0, vec![0..9]),
                (1, vec![0..9]),
                (2, vec![0..9]),
                (3, vec![0..9]),
                (4, vec![0..9]),
                (5, vec![0..9]),
                (6, vec![0..9]),
                (7, vec![0..9]),
                (8, vec![0..9]),
                (9, vec![0..9])
            ].iter().cloned().collect()    // To map for comparison
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
            [
                (1, vec![1..6, 7..10]),
                (2, vec![1..10]),
                (3, vec![1..10]),
                (4, vec![2..10]),
                (5, vec![2..10]),
                (6, vec![2..10])
            ].iter().cloned().collect()    // To map for comparison
        );
    }

    #[test]
    fn scanlines_merge() {
        assert_eq!(
            merge_and_order_scanlines(vec![
                [
                    (0, vec![0..33]),
                    (1, vec![1..5]),
                    (3, vec![0..50, 4..7])
                ].iter().cloned().collect(),
                [
                    (1, vec![0..6]),
                    (2, vec![2..8]),
                    (3, vec![1..20, 1..3])
                ].iter().cloned().collect()
            ]),
            [
                (0, vec![0..33]),
                (1, vec![0..6, 1..5]),
                (2, vec![2..8]),
                (3, vec![0..50, 1..20, 1..3, 4..7])
            ].iter().cloned().collect()
        );
    }
}