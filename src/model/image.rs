use windows::Win32::Graphics::Gdi::{BITMAPFILEHEADER, BITMAPINFOHEADER, RGBQUAD};

/// # Image
/// ## file_header
/// file header 14bit
/// ## info_header
/// info header 48bit
/// ## palette
/// color note? (variable bit)
/// ## body
/// real bitmap file (variable bit)
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
