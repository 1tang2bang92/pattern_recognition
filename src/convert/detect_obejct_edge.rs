use crate::model::Image;

impl Image {
    pub fn detect_object_edge(self) -> Self {
        let height = self.info_header.biHeight;
        let width = self.info_header.biWidth;
        let image_size: i32 = self.info_header.biWidth * self.info_header.biHeight;
        let mut body = vec![255; image_size as usize];

        for i in 0..height {
            for j in 0..width {
                if self.body[i * width +j] == 0 &&
                    !(
                        self.body[(i-1) * width +j] == 0 &&
                        self.body[(i+1) * width +j] == 0 &&
                        self.body[i * width + j-1] == 0 &&
                        self.body[i * width + j+1] == 0
                    ) {
                    body[i * width + j] = 0
                }
            }
        }

        Self {
            body,
            ..self
        }
    }
}