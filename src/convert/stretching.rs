use crate::model::Image;

impl Image {
    pub fn histogram_stretching(self, histogram: Vec<u32>) -> Self {
        let body = self.body;
        let indexed_histogram: Vec<(usize, u32)> = histogram.into_iter().enumerate().collect();

        let low: f64 = indexed_histogram
            .iter()
            .find(|(_, y)| y > &0)
            .map(|(x, _)| *x as u8)
            .unwrap_or(0)
            .into();

        let high: f64 = indexed_histogram
            .iter()
            .rev()
            .find(|(_, y)| y > &0)
            .map(|(x, _)| *x as u8)
            .unwrap_or(255)
            .into();

        let body = body
            .iter()
            .map(|x| ((x as f64 - low) / (high - low) * 255f64) as u8)
            .collect();

        Self {
            body,
            ..self
        }
    }
}
