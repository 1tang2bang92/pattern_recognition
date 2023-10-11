use crate::model::Image;

impl Image {
    pub fn contrast(self, contrast: f64) -> Self {
        let body = self.body
            .iter()
            .map(|x | {
                let a = x as f64 * contrast;
                if a > u8::MAX as f64 {
                    255
                } else if a < u8::MIN as f64 {
                    0
                } else {
                    a
                }
            })
            .collect();

        Self { body, ..self }
    }
}