// 膨張

use image::{GrayImage, ImageReader, Luma};

fn has_white(img: &GrayImage, x: u32, y: u32) -> bool {
    // (x, y)の周囲9マスに255があるか
    for ny in (y - 1)..(y + 2) {
        for nx in (x - 1)..(x + 2) {
            if img.get_pixel(nx, ny)[0] == 255 {
                return true;
            }
        }
    }

    false
}

fn main() {
    let ori_img = ImageReader::open("images/cat_binary.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();

    let (width, height) = ori_img.dimensions();

    println!("width: {width}, height: {height}");

    let mut img = ori_img.clone();

    // 1 ≤ x < w - 1, 1 ≤ y < h - 1
    // 端っこを含めないため
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            // 周囲9マスに255があれば白にする。

            let pixel: u8 = if has_white(&ori_img, x, y) { 255 } else { 0 };

            img.put_pixel(x, y, Luma([pixel]));
        }
    }

    std::fs::create_dir_all("out").unwrap();
    img.save("out/dilation.png").unwrap();
}
