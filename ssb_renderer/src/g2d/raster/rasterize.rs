// Imports
use crate::g2d::vector::{
    types::Coordinate,
    point::{Point,PointMinMaxCollector,ORIGIN_POINT},
    path::FlatPath
};
use super::{
    mask::Mask,
    scanlines::{scanlines_from_path,merge_and_order_scanlines}
};


// Sampling configuration
const SAMPLE_DEVIATIONS_NUMBER: usize = 8;
const SAMPLE_DEVIATIONS: [Point;SAMPLE_DEVIATIONS_NUMBER] = [
    Point {x: 0.125, y: -0.375},     // Top-top-right
    Point {x: 0.375, y: -0.125},     // Top-right-right
    Point {x: 0.375, y: 0.125},     // Bottom-right-right
    Point {x: 0.125, y: 0.375},     // Bottom-bottom-right
    Point {x: -0.125, y: 0.375},     // Bottom-bottom-left
    Point {x: -0.375, y: 0.125},     // Bottom-left-left
    Point {x: -0.375, y: -0.125},     // Top-left-left
    Point {x: -0.125, y: -0.375}      // Top-top-left
];
const SAMPLE_WEIGHT: u8 = {let weight = 256 / SAMPLE_DEVIATIONS_NUMBER; weight - ((weight & 256) >> 8)} as u8;

// Rasterize path to mask
pub fn rasterize_path(path: &FlatPath, area_width: u16, area_height: u16) -> Option<Mask> {
    // Calculate path offset & dimensions for mask
    let (path_bounding, deviations_bounding) = (
        path.bounding()?,
        SAMPLE_DEVIATIONS.iter().min_max()?
    );
    let (path_offset_rounded, path_peak_rounded) = (
        (path_bounding.0 + deviations_bounding.0).round_half_down(),
        (path_bounding.1 + deviations_bounding.1).round()
    );
    let (path_offset_trimmed, path_peak_trimmed) = (
        path_offset_rounded.max(ORIGIN_POINT),
        path_peak_rounded.min(Point {x: area_width as Coordinate, y: area_height as Coordinate})
    );
    let path_dimensions = (
        path_peak_trimmed.x as u16 - path_offset_trimmed.x as u16,
        path_peak_trimmed.y as u16 - path_offset_trimmed.y as u16
    );
    // Calculate scanlines & mask
    let (mut mask, scanlines) = (
        Mask {
            x: path_offset_trimmed.x as u16,
            y: path_offset_trimmed.y as u16,
            width: path_dimensions.0,
            height: path_dimensions.1,
            data: vec![0; path_dimensions.0 as usize * path_dimensions.1 as usize]
        },
        merge_and_order_scanlines(
            SAMPLE_DEVIATIONS.iter()
            .map(|deviation: &Point| {
                let mut new_path = path.clone();
                new_path.translate(-path_offset_rounded.x + deviation.x, -path_offset_rounded.y + deviation.y);
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
            .for_each(|pixel| *pixel = pixel.saturating_add(SAMPLE_WEIGHT) );
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
                data: vec![255]
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
                data: vec![64,64,64,64]
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
                    0, 64, 160, 255, 255, 160, 64, 0,
                    32, 255, 255, 255, 255, 255, 255, 32,
                    160, 255, 255, 255, 255, 255, 255, 160,
                    255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255,
                    160, 255, 255, 255, 255, 255, 255, 160,
                    32, 255, 255, 255, 255, 255, 255, 32,
                    0, 64, 160, 255, 255, 160, 64, 0
                ]
            })
        );
    }
}