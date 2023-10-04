use crate::kernel::{AVG, GAUSSIAN, PREWITT_X, PREWITT_Y, SEBEL_X, SEBEL_Y};
use crate::model::Image;

impl Image {
    pub fn convolution(&mut self, method: &str) -> &mut Image {
        let height = self.info_header.biHeight;
        let width = self.info_header.biWidth;
        let kernel = match method {
            "average" => AVG,
            "gaussian" => GAUSSIAN,
            "prewitt-x" => PREWITT_X,
            "prewitt-y" => PREWITT_Y,
            "sebel-x" => SEBEL_X,
            "sebel-y" => SEBEL_Y,
            _ => panic!("Invalid method")
        };

        for i in 0..height as usize {
            for j in 0..width as usize {

                let mut sum_product = 0.0;

                for m in 0..3 as usize {
                    for n in 0..3 as usize {
                        sum_product += self.body[(i + m) * width as usize + (j + n)] as f64 * kernel[m][n];
                    }
                }

                self.body[i * width as usize + j] = (sum_product.abs() / 4.0) as u8;
            }
        }

        self
    }
}