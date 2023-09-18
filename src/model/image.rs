use windows::Win32::Graphics::Gdi::{BITMAPFILEHEADER, BITMAPINFOHEADER, RGBQUAD};

pub struct Image {
    pub file_header: BITMAPFILEHEADER,
    pub info_header: BITMAPINFOHEADER,
    pub palette: Vec<RGBQUAD>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub enum ImageError {
    FileReadError,
    FileHeaderReadError,
    FileInfoHeaderReadError,
    PaletteReadError,
}
