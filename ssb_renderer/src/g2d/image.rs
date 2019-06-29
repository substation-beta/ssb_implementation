// Imports
use super::error::GraphicsError;


/// Supported color types for images.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorType {
    RGB,
    BGR,
    RGBA,
    BGRA
}
impl ColorType {
    /// Size of color (=bytes per pixel).
    pub fn bytes(&self) -> u8 {
        match &self {
            ColorType::RGB | ColorType::BGR => 3,
            ColorType::RGBA | ColorType::BGRA => 4
        }
    }
    /// Image stride by given color (=bytes per image row).
    pub fn stride(&self, width: u16) -> u32 {
        self.bytes() as u32 * width as u32
    }
    /// Color has alpha?
    pub fn alpha(&self) -> bool {
        match self {
            ColorType::RGB | ColorType::BGR => false,
            ColorType::RGBA | ColorType::BGRA => true
        }
    }
    /// Usual color channels are swapped (f.e. red and blue)?
    pub fn swapped(&self) -> bool {
        match self {
            ColorType::RGB | ColorType::RGBA => false,
            ColorType::BGR | ColorType::BGRA => true
        }
    }
}

/// Image with dimension, stride, color type and data buffer.
#[derive(Debug, PartialEq, Clone)]
pub struct Image {
    width: u16,
    height: u16,
    stride: u32,
    color_type: ColorType,
    data: Vec<u8>
}
impl Image {
    /// New blank image with default stride.
    pub fn new(width: u16, height: u16, color_type: ColorType) -> Self {
        Self::new_with_stride(width, height, color_type.stride(width), color_type).expect("Cannot fail with default stride & buffer!")
    }
    /// New blank image with user-defined stride.
    pub fn new_with_stride(width: u16, height: u16, stride: u32, color_type: ColorType) -> Result<Self,GraphicsError> {
        Self::from_data_with_stride(width, height, stride, color_type, vec![0u8; height as usize * stride as usize])
    }
    /// Image from existing buffer with default stride.
    pub fn from_data(width: u16, height: u16, color_type: ColorType, data: Vec<u8>) -> Result<Self,GraphicsError> {
        Self::from_data_with_stride(width, height, color_type.stride(width), color_type, data)
    }
    /// Image from existing buffer with user-defined stride.
    pub fn from_data_with_stride(width: u16, height: u16, stride: u32, color_type: ColorType, data: Vec<u8>) -> Result<Self,GraphicsError> {
        if stride < color_type.stride(width) {
            Err(GraphicsError::new("Stride too small, must cover <width * color size>!"))
        } else if data.len() < height as usize * stride as usize {
            Err(GraphicsError::new("Data buffer not big enough!"))
        } else {
            Ok(Self {
                width, height, stride: stride, color_type, data
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
    /// Get image stride (=width * color size + offset).
    pub fn stride(&self) -> u32 {
        self.stride
    }
    /// Get image color type.
    pub fn color_type(&self) -> ColorType {
        self.color_type
    }
    /// Get image data readable reference.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    /// Get image data mutable reference.
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
impl Into<Vec<u8>> for Image {
    fn into(self) -> Vec<u8> {
        self.data
    }
}