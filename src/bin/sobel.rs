// Sobelフィルタ

use image::{GrayImage, ImageReader, Luma};

fn apply_filter(img: &GrayImage, kernel: &[[i32; 3]; 3], x: u32, y: u32) -> i32 {
    // 周囲9マスとカーネルの積和を求める

    let mut sum: i32 = 0;
    for ny in (y - 1)..(y + 2) {
        for nx in (x - 1)..(x + 2) {
            let kx = (nx - (x - 1)) as usize;
            let ky = (ny - (y - 1)) as usize;

            let pixel = img.get_pixel(nx, ny)[0] as u32;
            sum += pixel as i32 * kernel[ky][kx];
        }
    }

    return sum;
}

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
    // X方向（水平エッジ検出）
    let kernel_x: [[i32; 3]; 3] = [
        [-1, 0, 1],
        [-2, 0, 2],
        [-1, 0, 1],
    ];

    #[rustfmt::skip]
    // Y方向（垂直エッジ検出）
    let kernel_y: [[i32; 3]; 3] = [
        [-1, -2, -1],
        [ 0,  0,  0],
        [ 1,  2,  1],
    ];

    // 1 ≤ x < w - 1, 1 ≤ y < h - 1
    // 端っこを含めないため
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {

            let gx = apply_filter(&ori_img, &kernel_x, x, y);
            let gy = apply_filter(&ori_img, &kernel_y, x, y);

            let g = (gx * gx + gy * gy).isqrt();

            // 0~255に収める
            let g_pixel = if g > 255 { 255 } else { g };

            img.put_pixel(x, y, Luma([g_pixel as u8]));
        }
    }

    img.save("images/out.png").unwrap();
}
