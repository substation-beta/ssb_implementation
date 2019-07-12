// Imports
use super::error::GraphicsError;


/// Color type for image data.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorType {
    RGB24,
    BGR24,
    R8G8B8
}
impl ColorType {
    /// Size of one color sample.
    pub fn sample_size(&self) -> u8 {
        match self {
            ColorType::RGB24 | ColorType::BGR24 => 3,
            ColorType::R8G8B8 => 1
        }
    }
    /// Size of all color samples in one image row.
    pub fn row_size(&self, width: u16) -> u32 {
        self.sample_size() as u32 * width as u32
    }
    /// Number of color planes for a type.
    pub fn planes(&self) -> u8 {
        match self {
            ColorType::RGB24 | ColorType::BGR24 => 1,
            ColorType::R8G8B8 => 3
        }
    }
}

/// Reference on image data with meta information.
#[derive(Debug, PartialEq)]
pub struct ImageView<'data> {
    width: u16,
    height: u16,
    stride: u32,
    color_type: ColorType,
    planes: Vec<&'data mut [u8]>
}
impl<'data> ImageView<'data> {
    /// New image view on given data.
    pub fn new(width: u16, height: u16, stride: u32, color_type: ColorType, planes: Vec<&'data mut [u8]>) -> Result<Self, GraphicsError> {
        if stride < color_type.row_size(width) {
            Err(GraphicsError::new("Stride must at least cover row size!"))
        } else if planes.len() != color_type.planes() as usize {
            Err(GraphicsError::new("Number of planes doesn't fit color type!"))
        } else if planes.iter().map(|plane| plane.len()).min().unwrap_or(0) < stride as usize * height as usize {
            Err(GraphicsError::new("At least one plane isn't big enough for expected data size!"))
        } else {
            Ok(Self {
                width,
                height,
                stride,
                color_type,
                planes
            })
        }
    }
    /// Get image width.
    pub fn width(&self) -> u16 {
        self.width
    }
    /// Get image height.
    pub fn height(&self) -> u16 {
        self.height
    }
    /// Get image stride (=row size + offset).
    pub fn stride(&self) -> u32 {
        self.stride
    }
    /// Get image color type.
    pub fn color_type(&self) -> ColorType {
        self.color_type
    }
    /// Get image plane of color data as readable reference.
    pub fn plane(&self, index: u8) -> Option<&&'data mut [u8]> {
        self.planes.get(index as usize)
    }
    /// Get image plane of color data as mutable reference.
    pub fn plane_mut(&mut self, index: u8) -> Option<&mut &'data mut [u8]> {
        self.planes.get_mut(index as usize)
    }
    /// Get image plane of color data as readable row references without offsets.
    pub fn plane_rows(&self, index: u8) -> Option<impl Iterator<Item = &[u8]>> {
        self.plane(index).map(|data| {
            let row_size = self.color_type.row_size(self.width) as usize;
            data.chunks_exact(self.stride as usize).take(self.height as usize).map(move |row| &row[..row_size])
        })
    }
    /// Get image plane of color data as mutable row references without offsets.
    pub fn plane_rows_mut(&mut self, index: u8) -> Option<impl Iterator<Item = &mut [u8]>> {
        let (row_size, stride, height) = (self.color_type.row_size(self.width) as usize, self.stride, self.height);
        self.plane_mut(index).map(|data| {
            data.chunks_exact_mut(stride as usize).take(height as usize).map(move |row| &mut row[..row_size])
        })
    }
}