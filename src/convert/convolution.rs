use crate::kernel::{LAPLACIAN_HPF, AVG, GAUSSIAN, LAPLACIAN, PREWITT_X, PREWITT_Y, SOBEL_X, SOBEL_Y};
use crate::model::Image;

impl Image {
    pub fn convolution(self, method: &str) -> Self {
        let height = self.info_header.biHeight;
        let width = self.info_header.biWidth;
        let (kernel, divide_value) = match method {
            "average" => (AVG, None),
            "gaussian" => (GAUSSIAN, None),
            "prewitt-x" => (PREWITT_X, 3),
            "prewitt-y" => (PREWITT_Y, 3),
            "sobel-x" => (SOBEL_X, 4),
            "sobel-y" => (SOBEL_Y, 4),
            "laplacian" => (LAPLACIAN, 8),
            "laplacian-hpf" => (LAPLACIAN_HPF, None),
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

                dst.body[(i * width + j) as usize] = sum_product.abs() as u8;
                if let v = Some(divide_value) {
                    dst.body[(i * width + j) as usize] = dst.body[(i * width + j) as usize] / v;
                }
            }
        }

        dst
    }
}