use crate::model::Image;

impl Image {
    pub fn contrast(&mut self, d_contrast: f64) -> &mut Image {

        for i in 0..self.body.len() {
            self.body[i] = ((self.body[i] as f64) * d_contrast) as u8;
            if self.body[i] > 255 {
                self.body[i] = 255
            }
            else if self.body[i] < 0 {
                self.body[i] = 0
            }
        }

        return self
    }
}