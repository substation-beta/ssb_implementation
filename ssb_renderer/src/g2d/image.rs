// Imports
use super::error::GraphicsError;


/// Color type for image data.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorType {
    // RGB
    RGB24,
    BGR24,
    R8G8B8,
    // RGBA
    RGBA32,
    ABGR32,
    R8G8B8A8
}
impl ColorType {
    /// Size of one color sample.
    pub fn sample_size(&self) -> u8 {
        match self {
            Self::RGB24 | Self::BGR24 => 3,
            Self::RGBA32 | Self::ABGR32 => 4,
            Self::R8G8B8 | Self::R8G8B8A8 => 1
        }
    }
    /// Size of all color samples in one image row.
    pub fn row_size(&self, width: u16) -> u32 {
        self.sample_size() as u32 * width as u32
    }
    /// Number of color planes for a type.
    pub fn planes(&self) -> u8 {
        match self {
            Self::RGB24 | Self::BGR24 | Self::RGBA32 | Self::ABGR32 => 1,
            Self::R8G8B8 => 3,
            Self::R8G8B8A8 => 4
        }
    }
    /// Color contains alpha channel?
    pub fn alpha(&self) -> bool {
        match self {
            Self::RGB24 | Self::BGR24 | Self::R8G8B8 => false,
            Self::RGBA32 | Self::ABGR32 | Self::R8G8B8A8 => true
        }
    }
    /// Usual color channels are swapped?
    pub fn swapped(&self) -> bool {
        match self {
            Self::RGB24 | Self::RGBA32 | Self::R8G8B8 | Self::R8G8B8A8 => false,
            Self::BGR24 | Self::ABGR32 => true
        }
    }
    /// Get variant by name.
    pub fn by_name(name: &str) -> Result<Self, GraphicsError> {
        match name.to_uppercase().as_str() {
            "RGB24" => Ok(Self::RGB24),
            "BGR24" => Ok(Self::BGR24),
            "R8G8B8" => Ok(Self::R8G8B8),
            "RGBA32" => Ok(Self::RGBA32),
            "ABGR32" => Ok(Self::ABGR32),
            "R8G8B8A8" => Ok(Self::R8G8B8A8),
            _ => Err(GraphicsError::new(&format!("'{}' isn't a valid color type!", name)))
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

// Tests
#[cfg(test)]
mod tests {
    use super::ColorType;

    #[test]
    fn color_type_by_name() {
        assert_eq!(ColorType::by_name("rgb24").expect("RGB24 name wasn't valid?!"), ColorType::RGB24);
        assert_eq!(ColorType::by_name("BGR24").expect("BGR24 name wasn't valid?!"), ColorType::BGR24);
        assert_eq!(ColorType::by_name("r8g8b8").expect("R8G8B8 name wasn't valid?!"), ColorType::R8G8B8);
        assert_eq!(ColorType::by_name("RGBA32").expect("RGBA32 name wasn't valid?!"), ColorType::RGBA32);
        assert_eq!(ColorType::by_name("abgr32").expect("ABGR32 name wasn't valid?!"), ColorType::ABGR32);
        assert_eq!(ColorType::by_name("r8g8b8a8").expect("R8G8B8A8 name wasn't valid?!"), ColorType::R8G8B8A8);
        assert!(ColorType::by_name("yuv").is_err());
    }
}