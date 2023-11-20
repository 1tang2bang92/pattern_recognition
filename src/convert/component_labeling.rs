use crate::model::Image;

impl Image {
    pub fn component_labeling(mut self, mode: i32) -> Self {
        let height = self.info_header.biHeight as usize;
        let width = self.info_header.biWidth as usize;

        let mut p_coloring = vec![0u8; height * width]; //
        let mut blob_area = vec![0; 1000];
        let mut p_stack_x = vec![0i16; height * width];
        let mut p_stack_y = vec![0i16; height * width];
        let mut cur_color: i32 = 0; //선택한 색? index?
        let mut out_area: i32 = 1;

        'outer: for i in 0..height {
            let index = i * width;
            for j in 0..width {
                if p_coloring[index + j] != 0 || self.body[index + j] != 255 {
                    continue;
                }

                let mut row = i as i16;
                let mut column = j as i16;
                let mut top = 0;
                let mut area = 1;
                cur_color += 1;

                'grassfire: loop {
                    for m in row -1..=row +1 {
                        let m_index = m as usize * width;
                        for n in column -1..=column +1 {
                            if m < 0 || m >= height as i16 || n < 0 || n >= width as i16 {
                                continue;
                            }

                            let n_index = n as usize;
                            if self.body[m_index + n_index] == 255 && p_coloring[m_index + n_index] == 0 {
                                p_coloring[m_index + n_index] = cur_color as u8;
                                if Image::push(&mut p_stack_x, &mut p_stack_y, height, m, n, &mut top) == -1 {
                                    continue;
                                }
                                row = m;
                                column = n;
                                area += 1;
                                break 'grassfire;
                            }
                        }
                    }
                    if Image::pop(&mut p_stack_x, &mut p_stack_y, &mut row, &mut column, &mut top) == -1 {
                        break 'outer;
                    }
                }

                if cur_color < 1000 {
                    blob_area[cur_color as usize] = area;
                }
            }
        }

        let gray_gap = 255.0 / cur_color as f32;

        for i in 1..=cur_color as usize {
            if blob_area[i] >= blob_area[out_area as usize] {
                out_area = i as i32;
            }
        }

        for k in self.body.iter_mut() {
            *k = 255;
        }

        for (k , color) in p_coloring.iter().zip(self.clone().body.iter()) {
            match mode {
                1 if color.to_owned() == out_area as u8 => self.body[k.to_owned() as usize] = 0,
                2 if blob_area[color.to_owned() as usize] > 500 => self.body[k.to_owned() as usize] = 0,
                3 => self.body[k.to_owned() as usize] = (color.to_owned() as f32 * gray_gap) as u8,
                _ => println!("Labeling Mode Error"),
            }
        }

        self
    }

    fn push(p_stack_x: &mut Vec<i16>, p_stack_y: &mut Vec<i16>, arr_size: usize, x: i16, y: i16, top: &mut i16) -> i32 {
        if (*top as usize) < arr_size {
            p_stack_x[*top as usize] = x;
            p_stack_y[*top as usize] = y;
            *top += 1;
            0
        } else {
            -1
        }
    }

    fn pop(p_stack_x: &mut Vec<i16>, p_stack_y: &mut Vec<i16>, r: &mut i16, c: &mut i16, top: &mut i16) -> i32 {
        if *top >= 0 {
            *top -= 1;
            *r = p_stack_x[*top as usize];
            *c = p_stack_y[*top as usize];
            0
        } else {
            -1
        }
    }
}