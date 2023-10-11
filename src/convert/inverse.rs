use crate::model::Image;

impl Image {
    pub fn inverse(self) -> Self {
        let mut body = self.body;

        for i in 0..body.len() {
            body[i] = u8::MAX - body[i];
        }

        Self { body, ..self }
    }
}