use crate::model::Image;

impl Image {
    pub fn brightness(&mut self, n_brightness: u8) -> &mut Image {

        for i in 0..self.body.len() {
            let a: i32 = self.body[i] as i32 + n_brightness as i32;
            self.body[i] = if a > 255 { 255 }
            else if a < 0 { 0 }
            else { a as u8 }
        }

        return self
    }
}