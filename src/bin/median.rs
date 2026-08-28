// 中央値フィルタ

use image::{ImageReader, Luma};

fn main() {
    let ori_img = ImageReader::open("images/cat_gray.png")
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
            // 周囲9マスの中央値を求める

            let mut pixels = Vec::new();
            for _ny in (y - 1)..(y + 2) {
                for _nx in (x - 1)..(x + 2) {
                    pixels.push(img.get_pixel(x, y)[0]);
                }
            }

            pixels.sort();

            let median = pixels[4];

            img.put_pixel(x, y, Luma([median as u8]));
        }
    }

    std::fs::create_dir_all("out").unwrap();
    img.save("out/median.png").unwrap();
}
