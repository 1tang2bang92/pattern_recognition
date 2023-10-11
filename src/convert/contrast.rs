use crate::model::Image;

impl Image {
    pub fn contrast(self, contrast: f64) -> Self {

        let mut body = self.body;

        for i in 0..body.len() {
            let temp = (body[i] as f64) * contrast;
            
            if temp > u8::MAX as f64 {
                body[i] = 255
            } else if temp < u8::MIN as f64 {
                body[i] = 0
            } else {
                body[i] = temp as u8;
            }
        }

        Self { body, ..self }
    }
}