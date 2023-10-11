mod convert;
mod model;
mod kernel;

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
    println!("7.  Histogram Stretching");
    println!("8.  Histogram Equalization");
    println!("9.  Average Convolution");
    println!("10. Gaussian Convolution");
    println!("11. Laplacian Convolution");
    println!("12. Prewitt X Convolution");
    println!("13. Prewitt Y Convolution");
    println!("14. Prewitt Convolution");
    println!("15. Sobel X Convolution");
    println!("16. Sobel Y Convolution");
    println!("17. Sobel Convolution");
    println!("18. Laplacian High Pass Filter Convolution");
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
            let histogram = image.clone().histogram();
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
        7 => { // Histogram stretching
            let histogram = image.clone().histogram();

            image
                .histogram_stretching(histogram)
                .save(save_file_name.trim().to_owned());
        },
        8 => { // Histogram equalization
            let hitogram = image.clone().histogram();

            image
                .histogram_equalization(hitogram)
                .save(save_file_name.trim().parse().unwrap());
        },
        9 => { // Average Convolution
            image
                .convolution("average")
                .save(save_file_name.trim().parse().unwrap())
        },
        10 => { // Gaussian Convolution
            image
                .convolution("gaussian")
                .save(save_file_name.trim().parse().unwrap())
        },
        11 => { // Laplacian Convolution
            image
                .convolution("laplacian")
                .save(save_file_name.trim().parse().unwrap())
        },
        12 => { // Prewitt X Convolution
            image
                .convolution("prewitt-x")
                .save(save_file_name.trim().parse().unwrap())
        },
        13 => { // Prewitt Y Convolution
            image
                .convolution("prewitt-y")
                .save(save_file_name.trim().parse().unwrap())
        },
        14 => { // Prewitt Convolution
            let x = image.clone().convolution("prewitt-x").body;
            let y = image.clone().convolution("prewitt-y").body;

            image.body = x
                .iter()
                .zip(y)
                .map(|(&a, b)| a.max(b))
                .collect::<Vec<u8>>();

            image
                .save(save_file_name.trim().parse().unwrap())
        },
        15 => { // Sobel X Convolution
            image
                .convolution("sobel-x")
                .save(save_file_name.trim().parse().unwrap())
        },
        16 => { // Sobel Y Convolution
            image
                .convolution("sobel-y")
                .save(save_file_name.trim().parse().unwrap())
        },
        17 => { // Sobel Convolution
            let x = image.clone().convolution("sobel-x").body;
            let y = image.clone().convolution("sobel-y").body;

            image.body = x
                .iter()
                .zip(y)
                .map(|(&a, b)| a.max(b))
                .collect::<Vec<u8>>();

            image
                .save(save_file_name.trim().parse().unwrap())
        },
        18 => { // Laplacian High Pass Filter
            image
                .convolution("laplacian-hpf")
                .save(save_file_name.trim().parse().unwrap())
        },
        _ => {
            println!("입력 값이 잘못되었습니다.")
        }
    }
}
