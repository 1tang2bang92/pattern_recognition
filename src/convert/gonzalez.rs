use crate::model::Image;

impl Image {
    pub fn gonzalez(&self, histogram: Vec<u32>) -> u8 {
        const EPSILON: i32 = 2;

        let indexed_histogram: Vec<(usize, u32)> = histogram
            .into_iter()
            .enumerate()
            .collect();

        let low: u8 = indexed_histogram
            .iter()
            .find(|(_, y)| y > &0)
            .map(|(x, _)| *x as u8)
            .unwrap_or(0);

        let high: u8 = indexed_histogram
            .iter()
            .rev()
            .find(|(_, y)| y > &0)
            .map(|(x, _)| *x as u8)
            .unwrap_or(255);

        let mut threshold: u8 = (high + low) / 2;

        fn get_mean (histogram_part: &[(usize, u32)]) -> f64 {
            let g: u32 = histogram_part
                .iter()
                .fold(0, |acc, &(i, x)| ((i as u32) * x) + acc);

            let len: u32 = histogram_part
                .iter()
                .fold(0, |acc, &(_, x)| { acc + (x as u32) });

            if len > 0 {
                f64::from(g) / (len as f64)
            } else {
                1.0
            }

        }

        loop {
            let mean_g1: f64 = get_mean(&indexed_histogram[low.into()..threshold.into()]);
            let mean_g2: f64 = get_mean(&indexed_histogram[threshold.into()..high.into()]);

            let new_threshold: u8 = ((mean_g1 + mean_g2) / 2.0) as u8;

            if ((threshold as i32) - (new_threshold as i32)).abs() < EPSILON {
                threshold = new_threshold;
                println!("Final Threshold = {}", new_threshold);
                break
            }
            else {
                threshold = new_threshold;
                println!("New Threshold = {}", new_threshold);
            }

        }

        threshold
    }
}
