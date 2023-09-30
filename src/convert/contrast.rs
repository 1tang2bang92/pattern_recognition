use crate::model::Image;

impl Image {
    pub fn contrast(&mut self, contrast: f64) -> &mut Self {

        for i in 0..self.body.len() {
            let temp = (self.body[i] as f64) * contrast;
            
            if self.body[i] > u8::MAX {
                self.body[i] = 255
            }
            else if self.body[i] < u8::MIN {
                self.body[i] = 0
            }
            else {
                self.body[i] = temp as u8;
            }
        }

        return self
    }
}