mod convert;
mod model;

use std::io;
use crate::model::Image;

fn main() {
    println!("=================================\n");
	println!("Image Processing Program\n\n");
	println!("1.  Inverse\n");
	println!("=================================\n\n");
    
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
        .map_err(|err| eprintln!("{:?}", err))
        .unwrap();

    match n_mode {
        1 => image.inverse().save(),
        _ => {}
    }
}
