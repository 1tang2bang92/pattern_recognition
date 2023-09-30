use crate::model::Image;

impl Image {
    pub fn histogram_stretching(&mut self, histogram: Vec<u32>) -> &mut Self {
        let image_size = self.info_header.biHeight * self.info_header.biWidth;

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

        for i in 0..image_size {
            self.body[i] = (self.body[i] - low) / (high - low) * 255;
        }

        self
    }
}