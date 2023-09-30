use crate::model::Image;

impl Image {
    pub fn binarization(mut self, threshold: u8) -> Self {
        self.body = self.body
            .iter()
            .map(|x | if x > &threshold {
                255
            } else {
                0
            })
            .collect();
        
        self
    }
}