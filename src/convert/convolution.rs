use crate::kernel::{AVG, GAUSSIAN, LAPLACIAN, PREWITT_X, PREWITT_Y, SEBEL_X, SEBEL_Y};
use crate::model::Image;

impl Image {
    pub fn convolution(&self, method: &str) -> Self {
        let height = self.info_header.biHeight;
        let width = self.info_header.biWidth;
        let kernel = match method {
            "average" => AVG,
            "gaussian" => GAUSSIAN,
            "prewitt-x" => PREWITT_X,
            "prewitt-y" => PREWITT_Y,
            "sebel-x" => SEBEL_X,
            "sebel-y" => SEBEL_Y,
            "laplacian" => LAPLACIAN,
            _ => panic!("Invalid method")
        };

        let mut dst = self.clone();

        for i in 1..height-1 {
            for j in 1..width-1 {

                let mut sum_product = 0.0;

                for m in -1..2 {
                    for n in -1..2 {
                        sum_product += 
                            self.body[
                                ((i + m) * width + (j + n)) as usize
                            ] as f64 
                            * kernel[(m + 1) as usize][(n + 1) as usize];
                    }
                }

                dst.body[(i * width + j) as usize] = sum_product.abs() as u8 / 4;
            }
        }

        dst
    }
}