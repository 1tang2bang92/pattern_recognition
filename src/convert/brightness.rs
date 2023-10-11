use crate::model::Image;

impl Image {
    pub fn brightness(self, n_brightness: u8) -> Self {

        let body = self.body
            .iter()
            .map(|x | {
                let a = (*x as i32) + (n_brightness as i32);
                if a > 255 {
                    255
                } else if a < 0 {
                    0
                } else {
                    a as u8
                }
            })
            .collect();

        Self { body, ..self }
    }
}