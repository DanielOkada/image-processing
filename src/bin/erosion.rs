// 縮小

use image::{GrayImage, ImageReader, Luma};

fn has_black(img: &GrayImage, x: u32, y: u32) -> bool {
    // (x, y)の周囲9マスに0があるか
    for ny in (y - 1)..(y + 2) {
        for nx in (x - 1)..(x + 2) {
            if img.get_pixel(nx, ny)[0] == 0 {
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
            // 周囲9マスに黒が1つでもあれば黒にする。

            let pixel: u8 = if has_black(&ori_img, x, y) { 0 } else { 255 };

            img.put_pixel(x, y, Luma([pixel]));
        }
    }

    img.save("out/erosion.png").unwrap();
}
