use crate::model::Image;

impl Image {
    pub fn brightness(self, n_brightness: u8) -> Self {

        let body = self.body
            .iter()
            .map(|x | {
                let a = x + n_brightness;
                if a > 255 {
                    255
                } else if a < 0 {
                    0
                } else {
                    a
                }
            })
            .collect();

        Self { body, ..self }
    }
}