// Imports
use super::{
    path::{PathBase,Path,SubPathIterator}
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
    // Any width/filling?
    let width = width_x.max(width_y);
    if width > 0.0 {
        // Width weights
        let (width_weight_x, width_weight_y) = if width_x > width_y {
            (1.0, width_y.max(0.0) / width_x)
        } else {
            (width_x.max(0.0) / width_y, 1.0)
        };
        // Build offset path
        let stroke_path_segments = vec![];
        for sub_path in SubPathIterator::new(path.segments()) {

            // TODO: prepare join & cap function (or macro?)

            // TODO: create & connect offset paths

        }
        // Return collected segments as path
        Path::new(stroke_path_segments)
    } else {
        Path::default()
    }
}


// Tests
#[cfg(test)]
mod tests {
}