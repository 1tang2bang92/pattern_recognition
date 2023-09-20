use crate::model::Image;

impl Image {
    pub fn inverse(&mut self) -> &mut Image {

        for i in 0..self.body.len() {
            self.body[i] = u8::MAX - self.body[i];
        }

        return self
    }
}