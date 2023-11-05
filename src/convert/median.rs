use crate::model::Image;

impl Image {
    pub fn median(self, method: &str, kernel_size: i32) -> Self {
        let height = self.info_header.biHeight;
        let width = self.info_header.biWidth;

        let min_kernel_index = -kernel_size / 2;
        let max_kernel_index = kernel_size / 2 + 1;
        let kernel_rage = min_kernel_index..max_kernel_index;

        let mode = match method {
            "min" => |x: Vec<u8>| x[0],
            "max" => |x: Vec<u8>| x[x.len() -1],
            "median" => |x: Vec<u8>| x[x.len() / 2],
            _ => panic!("Invalid method")
        };

        let mut dst = self.clone();

        for i in 1..width {
            for j in 1..height {
                let mut tmp = vec!();
                for m in kernel_rage.clone() {
                    for n in kernel_rage.clone() {
                        let x = i + m;
                        let y = j + n;
                        tmp.push(self.body[(x * width + y) as usize]);
                    }
                }
                tmp.sort();
                dst.body[(i * width + j) as usize] = mode(tmp);
            }
        }

        dst
    }
}