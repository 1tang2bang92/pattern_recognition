use std::iter::Map;
use std::slice::Iter;
use crate::model::Image;

impl Image {
    pub fn gonzalez(self, epsilon: f64) {
        let mut hitogram: Vec<u32> = vec![0,256];

        for pixel in self.body {
            hitogram[pixel] += 1
        }

        none
    }

    fn gonzalez_t(self, pixel_count: u32, histogram: Vec<u32>) {

    }
}