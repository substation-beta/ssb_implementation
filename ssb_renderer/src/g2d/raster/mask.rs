#[derive(Debug,PartialEq,Clone)]
pub struct Mask {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>
}