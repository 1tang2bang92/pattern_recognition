use crate::model::Image;

impl Image {
    pub fn histogram_equalization(self, histogram: Vec<u32>) -> Self {
        let mut body = self.body;
        let image_size =  self.info_header.biHeight * self.info_header.biWidth;
        let g_max = 256;

        let ratio: f64 = g_max as f64 / image_size as f64;
        let mut norm_sum: Vec<u8> = vec![0u8; 256usize];
        let mut a_histogram: Vec<i32> = vec![0i32; 256usize];

        for i in 0..256 {
            for j in 0..i {
                a_histogram[i] += histogram[j] as i32;
            }
        }
        for i in 0..256 {
            norm_sum[i as usize] = (ratio * a_histogram[i as usize] as f64) as u8;
        }
        for i in 0..image_size as usize {
            body[i] = norm_sum[body[i] as usize];
        }

        Self { body, ..self }
    }
}