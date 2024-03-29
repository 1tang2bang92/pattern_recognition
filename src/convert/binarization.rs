use crate::model::Image;

impl Image {
    pub fn binarization(self, threshold: u8) -> Self {
        let body = self.body
            .iter()
            .map(|x | if x > &threshold {
                255
            } else {
                0
            })
            .collect();
        
        Self { body, ..self }
    }
}