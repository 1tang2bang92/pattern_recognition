use crate::model::Image;

impl Image {
    pub fn inverse(&mut self) -> &mut Image {
        // let width = self.info_header.biWidth;
        // let height = self.info_header.biHeight;
        //
        // let bit_count = self.info_header.biBitCount;
        //
        // let byte_per_pixel = ((bit_count + 7) / 8) as i32;
        //
        // let stride = ((byte_per_pixel * width + 3) / 4) * 4;

        // Vec::with_capacity(self.body.len())
        //     .zip(&self.body)
        //     .map(|mut i, j| {
        //         i = 255 - j;
        //     });

        for i in 0..self.body.len() {
            self.body[i] = u8::MAX - self.body[i];
        }

        return self

        // println!("Width: {}, Height: {}, BitCount: {}, BytePerPixel: {}, Stride: {}", width, height, bit_count, byte_per_pixel, stride);
    }
}