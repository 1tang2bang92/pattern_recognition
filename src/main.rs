mod convert;
mod model;

use std::io;
use crate::model::Image;

fn main() {
    println!("=================================");
	println!("Image Processing Program");
	println!();
	println!("1.  Inverse");
	println!("2.  Adjust Brightness");
	println!("3.  Adjust Contrast");
	println!("4.  Generate Histogram");
	println!("5.  Generate Binarization - Gonzalez Method");
	println!("6.  Generate Binarization");
	println!("=================================");
	println!();
	println!();

	println!("원하는 기능의 번호를 입력하세요 : ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let n_mode: i32 = input.trim().parse().unwrap();

    input = (&"").parse().unwrap();
    println!("원본 이미지 파일의 경로를 입력하세요 : ");
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let path = input.trim().to_owned();
    let mut image: Image = Image::new(&path)
        .map_err(|err| println!("{:?}", err))
        .unwrap();


    println!("저장할 이미지 이름을 입력하세요 : ");
    let mut save_file_name = String::new();
    io::stdin()
        .read_line(&mut save_file_name)
        .unwrap();

    match n_mode {
        1 => image // Inverse
                .inverse()
                .save(save_file_name.trim().to_owned()),
        2 => { // Brightness
            println!("밝기 조절 값(정수)을 입력하세요 : ");
            let mut brightness_input = String::new();
            io::stdin()
                .read_line(&mut brightness_input)
                .unwrap();

            let brightness: u8 = brightness_input.trim().parse().unwrap();

            image
                .brightness(brightness)
                .save(save_file_name.trim().to_owned());
        },
        3 => { // Contrast
            println!("대비 조절 값(실수)을 입력하세요 : ");
            let mut contrast_input = String::new();
            io::stdin()
                .read_line(&mut contrast_input)
                .unwrap();

            let contrast: f64 = contrast_input.trim().parse().unwrap();

            image
                .contrast(contrast)
                .save(save_file_name.trim().to_owned());
        },
        4 => { // Histogram
            let histogram = image.histogram();
            println!("{:?}", histogram);
        },
        5 => { // Binarization using Gonzalez algorithm
            let histogram = image.histogram();
            let threshold = image.gonzalez(histogram);

            image
                .binarization(threshold)
                .save(save_file_name.trim().to_owned());
        },
        6 => { // Binarization
            println!("이진화 임계값(Threshold)를 입력하세요: ");
            let mut threshold_input = String::new();
            io::stdin()
                .read_line(&mut threshold_input)
                .unwrap();

            let threshold: u8 = threshold_input.trim().parse().unwrap();

            image
                .binarization(threshold)
                .save(save_file_name.trim().to_owned());
        },
        _ => {
            println!("입력 값이 잘못되었습니다.")
        }
    }
}
