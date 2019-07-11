// Imports
use super::error::GraphicsError;


/// Reference on image data with meta information.
#[derive(Debug, PartialEq)]
pub struct ImageView<'data> {
    width: u16,
    height: u16,
    stride: u32,
    channels: Channels<'data>
}
impl<'data> ImageView<'data> {
    // Construction template.
    fn new(width: u16, height: u16, stride: u32, channels: Channels<'data>) -> Result<Self,GraphicsError> {
        if stride < channels.row_size(width) {
            Err(GraphicsError::new("Stride too small, must cover row size!"))
        } else {
            Ok(Self {
                width,
                height,
                stride,
                channels
            })
        }
    }
    /// New image view on RGB24 data.
    pub fn new_rgb24(width: u16, height: u16, stride: u32, data: &'data mut [u8]) -> Result<Self,GraphicsError> {
        if data.len() < height as usize * stride as usize {
            Err(GraphicsError::new("Data buffer not big enough!"))
        } else {
            Self::new(width, height, stride, Channels::RGB24(data))
        }
    }
    /// New image view on BGR24 data.
    pub fn new_bgr24(width: u16, height: u16, stride: u32, data: &'data mut [u8]) -> Result<Self,GraphicsError> {
        if data.len() < height as usize * stride as usize {
            Err(GraphicsError::new("Data buffer not big enough!"))
        } else {
            Self::new(width, height, stride, Channels::BGR24(data))
        }
    }
    /// New image view on R8G8B8 data.
    pub fn new_r8g8b8(width: u16, height: u16, stride: u32, red_data: &'data mut [u8], green_data: &'data mut [u8], blue_data: &'data mut [u8]) -> Result<Self,GraphicsError> {
        let image_size = height as usize * stride as usize;
        if red_data.len() < image_size || green_data.len() < image_size || blue_data.len() < image_size {
            Err(GraphicsError::new("Data buffer not big enough!"))
        } else {
            Self::new(width, height, stride, Channels::R8G8B8(red_data, green_data, blue_data))
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
    /// Get image sample size.
    pub fn sample_size(&self) -> u8 {
        self.channels.sample_size()
    }
    /// Get image row size (=width * sample size).
    pub fn row_size(&self) -> u32 {
        self.channels.row_size(self.width)
    }
    /// Get image channels (data separated in color channels) as readable reference.
    pub fn channels(&mut self) -> &Channels<'data> {
        &self.channels
    }
    /// Get image channels (data separated in color channels) as mutable reference.
    pub fn channels_mut(&mut self) -> &mut Channels<'data> {
        &mut self.channels
    }


    // TODO: add convenient functions for data iteration
    /*
    /// Get channel data as mutable row references without offsets.
    pub fn channel_rows(&mut self) -> impl Iterator<Item = &mut [u8]> {
        let row_size = self.row_size() as usize;
        self.data.chunks_exact_mut(self.stride as usize).take(self.height as usize).map(move |row| &mut row[..row_size])
    }
    */

    
}

/// Color type & channels.
#[derive(Debug, PartialEq)]
pub enum Channels<'data> {
    RGB24(&'data mut [u8]),
    BGR24(&'data mut [u8]),
    R8G8B8(&'data mut [u8], &'data mut [u8], &'data mut [u8])
}
impl<'data> Channels<'data> {
    // Size of channel samples (=color) in bytes.
    fn sample_size(&self) -> u8 {
        match self {
            Channels::RGB24(_) | Channels::BGR24(_) => 3,
            Channels::R8G8B8(_, ..) => 1
        }
    }
    // Size of channel row (samples in one stride) in bytes.
    fn row_size(&self, width: u16) -> u32 {
        self.sample_size() as u32 * width as u32
    }
}