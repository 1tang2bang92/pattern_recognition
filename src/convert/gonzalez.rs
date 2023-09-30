use crate::model::Image;

impl Image {
    pub fn gonzalez(self, histogram: Vec<u32>) -> u8 {
        let low: u8 = histogram
            .iter()
            .enumerate()
            .find(|(_, &y)| y != 0)
            .map(|(x, _)| x as u8)
            .unwrap_or(0);

        let high: u8 = histogram
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &y)| y != 0)
            .map(|(x, _)| x as u8)
            .unwrap_or(0);

        const EPSILON: u8 = 2;
        let mut threshold: u8 = (high + low) / 2;

        fn get_mean (histogram_part: Vec<u32>) -> i32 {
            let g: i32 = histogram_part
                .iter()
                .enumerate()
                .map(|i, x| i * x)
                .sum();

            let len = histogram_part.len();
            if len > 0 {
                g / len
            } else {
                1
            }

        }

        loop {
            let mean_g1: i32 = get_mean(histogram[low..threshold]);
            let mean_g2: i32 = get_mean(histogram[threshold..high]);

            let new_threshold: u8 = ((mean_g1 + mean_g2) / 2) as u8;

            if (new_threshold - threshold).abs() < EPSILON {
                threshold = new_threshold;
                println!("Final Threshold = {}", new_threshold);
                break
            }
            else {
                threshold = new_threshold;
                println!("New Threshold = {}", new_threshold);
            }

        }

        return threshold
    }
}
