use crate::model::Image;

impl Image {
    pub fn inverse(self) -> Self {
        let body = self.body;

        let body = body
            .iter()
            .map(|x| u8::MAX - *x)
            .collect();

        Self { body, ..self }
    }
}