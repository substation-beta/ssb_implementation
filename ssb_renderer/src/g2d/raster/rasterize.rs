// Imports
use crate::g2d::vector::{
    types::Coordinate,
    point::{Point,PointMinMaxCollector,ORIGIN_POINT,GenericPoint},
    path::FlatPath
};
use super::{
    mask::Mask,
    scanlines::{scanlines_from_path,merge_and_order_scanlines}
};


// Sampling configuration
const SAMPLE_DEVIATIONS_NUMBER: usize = 8;
const SAMPLE_DEVIATIONS: [Point;SAMPLE_DEVIATIONS_NUMBER] = [
    Point {x: 0.125, y: -0.375},    // Top-top-right
    Point {x: 0.375, y: -0.125},    // Top-right-right
    Point {x: 0.375, y: 0.125},     // Bottom-right-right
    Point {x: 0.125, y: 0.375},     // Bottom-bottom-right
    Point {x: -0.125, y: 0.375},    // Bottom-bottom-left
    Point {x: -0.375, y: 0.125},    // Bottom-left-left
    Point {x: -0.375, y: -0.125},   // Top-left-left
    Point {x: -0.125, y: -0.375}    // Top-top-left
];
const SAMPLE_WEIGHT: f32 = 1.0 / SAMPLE_DEVIATIONS_NUMBER as f32;

// Rasterize path to mask
pub fn rasterize_path(path: &FlatPath, area_width: u16, area_height: u16) -> Option<Mask> {
    // Calculate path offset & dimensions for mask
    let (path_bounding, deviations_bounding) = (
        path.bounding()?,
        SAMPLE_DEVIATIONS.iter().min_max()?
    );
    let (path_offset, path_peak) = (
        (path_bounding.0 + deviations_bounding.0).round_half_down().max(ORIGIN_POINT),
        (path_bounding.1 + deviations_bounding.1).round().min(Point {x: area_width as Coordinate, y: area_height as Coordinate})
    );
    let path_dimensions = GenericPoint::<u16>::from(path_peak) - GenericPoint::<u16>::from(path_offset);
    // Calculate scanlines & mask
    let (mut mask, scanlines) = (
        Mask {
            x: path_offset.x as u16,
            y: path_offset.y as u16,
            width: path_dimensions.x,
            height: path_dimensions.y,
            data: vec![0.0; path_dimensions.x as usize * path_dimensions.y as usize]
        },
        merge_and_order_scanlines(
            SAMPLE_DEVIATIONS.iter()
            .map(|deviation: &Point| {
                let mut new_path = path.clone();
                new_path.translate(-path_offset.x + deviation.x, -path_offset.y + deviation.y);
                scanlines_from_path(&new_path, area_width, area_height)
            })
            .collect()
        )
    );
    // Rasterize scanlines on mask (addition with top-trim)
    mask.data.chunks_exact_mut(mask.width as usize)
    .enumerate()
    .filter_map(|(row_index, row_data)|
        scanlines.get(&(row_index as u16))
        .map(|scanline| (scanline, row_data) )
    )
    .for_each(|(scanline, row_data)|
        for range in scanline {
            row_data.iter_mut()
            .skip(range.start as usize)
            .take((range.end - range.start) as usize)
            .for_each(|pixel| *pixel += SAMPLE_WEIGHT );
        }
    );
    Some(mask)
}


// Tests
#[cfg(test)]
mod tests {
    use crate::g2d::vector::{
        point::Point,
        path::{PathBase,Path}
    };
    use super::{rasterize_path,FlatPath,Mask};

    #[test]
    fn rasterize_single_pixel() {
        // Pixel path
        let mut path = FlatPath::default();
        path.move_to(Point {x: 2.0, y: 3.0})
            .line_to(Point {x: 3.0, y: 3.0})
            .line_to(Point {x: 3.0, y: 4.0})
            .line_to(Point {x: 2.0, y: 4.0})
            .close();
        // Test
        assert_eq!(
            rasterize_path(&path, 5, 5),
            Some(Mask {
                x: 2,
                y: 3,
                width: 1,
                height: 1,
                data: vec![1.0]
            })
        );
        // Split to subpixels
        path.translate(-0.5, 0.5);
        // Test
        assert_eq!(
            rasterize_path(&path, 5, 5),
            Some(Mask {
                x: 1,
                y: 3,
                width: 2,
                height: 2,
                data: vec![0.25, 0.25, 0.25, 0.25]
            })
        );
    }

    #[test]
    fn rasterize_point() {
        // Path
        let mut path = Path::default();
        path.move_to(Point {x: 5.0, y: 1.0})
            .arc_by(Point {x: 5.0, y: 5.0}, 360.0)
            .close();
        // Test
        assert_eq!(
            rasterize_path(&FlatPath::from(path), 10, 10),
            Some(Mask {
                x: 1,
                y: 1,
                width: 8,
                height: 8,
                data: vec![
                    0.0, 0.25, 0.625, 1.0, 1.0, 0.625, 0.25, 0.0,
                    0.125, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.125,
                    0.625, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.625,
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                    0.625, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.625,
                    0.125, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.125,
                    0.0, 0.25, 0.625, 1.0, 1.0, 0.625, 0.25, 0.0
                ]
            })
        );
    }

    #[test]
    fn rasterize_simple_path() {
        // Path
        let mut path = Path::default();
        path.move_to(Point {x: -1.0, y: 10.0})
            .curve_to(Point {x: 5.0, y: 12.0}, Point {x: 15.0, y: 0.0}, Point {x: 20.0, y: 10.0})
            .line_to(Point {x: 0.0, y: 19.0})
            .close()
            .move_to(Point {x: 19.0, y: 17.0})
            .line_to(Point {x: 21.0, y: 17.0})
            .line_to(Point {x: 21.0, y: 18.0})
            .line_to(Point {x: 19.0, y: 18.0})
            .close();
        // Test
        assert_eq!(
            rasterize_path(&FlatPath::from(path), 20, 18),
            Some(Mask {
                x: 0,
                y: 6,
                width: 20,
                height: 12,
                data: vec![
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 0.75, 0.125, 0.0, 0.0, 0.0,
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.375, 0.875, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.25, 0.0, 0.0,
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.125, 0.625, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.375, 0.0,
                    0.0, 0.0, 0.125, 0.375, 0.625, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5,
                    1.0, 0.875, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.625, 0.375,
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.625, 0.125, 0.0, 0.0,
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.875, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.875, 0.375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.625, 0.375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.625, 0.125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    1.0, 1.0, 0.875, 0.375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0
                ]
            })
        );
    }
}