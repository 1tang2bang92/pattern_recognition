use crate::model::Image;

impl Image {
    pub fn brightness(self, n_brightness: u8) -> Self {

        let mut body = self.body;

        for i in 0..body.len() {
            let a: i32 = body[i] as i32 + n_brightness as i32;
            body[i] = if a > 255 {
                255
            } else if a < 0 {
                0
            } else {
                a as u8
            }
        }

        Self { body, ..self }
    }
}