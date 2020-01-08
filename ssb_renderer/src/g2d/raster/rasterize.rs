// Imports
use crate::g2d::vector::{
    point::{Point,PointMinMaxCollector},
    path::{PathBase,FlatPath}
};
use super::{
    mask::Mask,
    scanlines::{scanlines_from_path,merge_and_order_scanlines}
};
use rayon::prelude::*;


// Sampling configuration
const SAMPLE_DEVIATIONS_NUMBER: usize = 4;
const SAMPLE_DEVIATIONS: [Point;SAMPLE_DEVIATIONS_NUMBER] = [
    Point {x: -0.25, y: -0.25},
    Point {x: 0.25, y: -0.25},
    Point {x: 0.25, y: 0.25},
    Point {x: -0.25, y: 0.25}
];
const SAMPLE_WEIGHT: u8 = {let weight = 256 / SAMPLE_DEVIATIONS_NUMBER; weight - ((weight & 256) >> 8)} as u8;

// Rasterize path to mask
pub fn rasterize_path(path: &FlatPath, area_width: u16, area_height: u16) -> Option<Mask> {
    // Create mask
    let (path_bounding, deviations_bounding) = (path.bounding()?, SAMPLE_DEVIATIONS.iter().min_max()?);
    let (path_offset, path_peak) = (path_bounding.0 + deviations_bounding.0, path_bounding.1 + deviations_bounding.1);
    let _path_dimensions = path_peak - path_offset;

    // TODO

    // Calculate scanlines in parallel
    let path_to_deviated_scanlines = |deviation: &Point| {
        let mut new_path = path.clone();
        new_path.translate(deviation.x, deviation.y);
        scanlines_from_path(&new_path, area_width, area_height)
    };
    let scanlines = merge_and_order_scanlines(
        if path.segments().len() > 1000 {
            SAMPLE_DEVIATIONS.into_par_iter()
            .map(path_to_deviated_scanlines)
            .collect()
        } else {
            SAMPLE_DEVIATIONS.into_iter()
            .map(path_to_deviated_scanlines)
            .collect()
        }
    );
    // Rasterize scanlines on mask (addition with top-trim)
    for _scanline in scanlines {

        // TODO

    }
    None
}