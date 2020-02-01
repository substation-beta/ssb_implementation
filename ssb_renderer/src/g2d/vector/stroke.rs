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
    let stroke_path_segments = vec![];
    for sub_path in SubPathIterator::new(path.segments()) {

        // TODO: prepare join & cap function (or macro?)

        // TODO: create & connect offset paths

    }
    Path::new(stroke_path_segments)
}


// Tests
#[cfg(test)]
mod tests {
}