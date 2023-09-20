use std::fs::File;
use std::io::Write;
use std::mem::size_of;
use std::slice::from_raw_parts;
use crate::model::{Image};

unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    from_raw_parts((p as *const T) as *const u8,size_of::<T>())
}

impl Image {
    pub fn save(&self, save_file_name: String) {
        let mut f = File::create(format!("./result/{}.bmp", save_file_name)).unwrap();

        // 파일 헤더 작성
        let file_header_bytes = unsafe { as_u8_slice(&self.file_header) };
        f.write_all(&file_header_bytes).unwrap();

        // 정보 헤더 작성
        let info_header_bytes = unsafe { as_u8_slice(&self.info_header) };
        f.write_all(&info_header_bytes).unwrap();

        // 팔레트 작성
        for palette_color in &self.palette {
            let palette_entry_bytes = unsafe { as_u8_slice(palette_color) };
            f.write_all(&palette_entry_bytes).unwrap();
        }

        // 이미지 바디 작성
        f.write_all(&self.body).unwrap();

        f.flush().unwrap();
    }
}