pub mod inverse;
pub mod save;
pub mod brightness;
pub mod contrast;
mod gonzalez;

use std::fs::File;
use std::io::Read;
use std::mem;
use windows::Win32::Graphics::Gdi::{BITMAPFILEHEADER, BITMAPINFOHEADER, RGBQUAD};

use crate::model::Image;
use crate::model::ImageError;

impl Image {
    pub fn new(path: &String) -> Result<Image, ImageError> {
        let mut file = File::open(path).map_err(|_| ImageError::FileReadError)?;

        // 파일 헤더 읽기
        let mut file_header_buf = [0u8; mem::size_of::<BITMAPFILEHEADER>()];
        file.read_exact(&mut file_header_buf).map_err(|_| ImageError::FileHeaderReadError)?;

        let file_header: BITMAPFILEHEADER = unsafe { mem::transmute(file_header_buf) };

        // 정보 헤더 읽기
        let mut info_header_buf = [0u8; mem::size_of::<BITMAPINFOHEADER>()];
        file.read_exact(&mut info_header_buf)
            .map_err(|_| ImageError::FileInfoHeaderReadError)?;

        let info_header: BITMAPINFOHEADER = unsafe { mem::transmute(info_header_buf) };

        // 팔레트 데이터 읽기
        let palette_size = info_header.biClrUsed as usize;
        let mut palette = Vec::with_capacity(palette_size);

        for _ in 0..palette_size {
            let mut palette_entry = [0u8; mem::size_of::<RGBQUAD>()];
            file.read_exact(&mut palette_entry)
                .map_err(|_| ImageError::PaletteReadError)?;

            let palette_color: RGBQUAD = unsafe { mem::transmute(palette_entry) };
            palette.push(palette_color);
        }

        //body 읽기
        let mut body = Vec::new();
        file.read_to_end(&mut body)
            .map_err(|_| ImageError::FileReadError)?;

        Ok(Image {
            file_header,
            info_header,
            palette,
            body,
        })
    }

}
