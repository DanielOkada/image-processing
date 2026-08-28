// ガウシアンフィルタ

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

    #[rustfmt::skip]
    let kernel = [
        [1, 2, 1],
        [2, 4, 2],
        [1, 2, 1],
    ];

    let kernel_sum = 16;

    // 1 ≤ x < w - 1, 1 ≤ y < h - 1
    // 端っこを含めないため
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            // 周囲9マスの重み付き平均値を求める

            let mut sum: u32 = 0;
            for ny in (y - 1)..(y + 2) {
                for nx in (x - 1)..(x + 2) {
                    let kx = (nx - (x - 1)) as usize;
                    let ky = (ny - (y - 1)) as usize;

                    let pixel = ori_img.get_pixel(nx, ny)[0] as u32;
                    sum += pixel * kernel[ky][kx];
                }
            }

            let average = sum / kernel_sum;

            img.put_pixel(x, y, Luma([average as u8]));
        }
    }

    img.save("out/gaussian.png").unwrap();
}
