use crate::model::Image;

impl Image {
    pub fn histogram(self) -> Vec<u32> {
        let mut histogram: Vec<u32> = vec![0,256];
        for pixel in self.body {
            histogram[pixel] += 1
        }

        histogram
    }
}