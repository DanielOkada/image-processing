// Cannyフィルタ

use std::f32::consts::PI;

use image::{GrayImage, ImageBuffer, ImageReader, Luma};

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

fn gaussian_filter(img: &GrayImage) -> GrayImage {
    let mut blur_img = img.clone();
    let (width, height) = img.dimensions();

    #[rustfmt::skip]
    let kernel = [
        [1, 2, 1],
        [2, 4, 2],
        [1, 2, 1],
    ];

    let kernel_sum = 16;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            // 周囲9マスの重み付き平均値を求める

            let sum = apply_filter(&img, &kernel, x, y);
            let average = sum / kernel_sum;

            blur_img.put_pixel(x, y, Luma([average as u8]));
        }
    }

    blur_img
}

type IntImage = ImageBuffer<Luma<i32>, Vec<i32>>;
type FloatImage = ImageBuffer<Luma<f32>, Vec<f32>>;

fn sobel_filter(img: &GrayImage) -> (IntImage, FloatImage) {
    let (width, height) = img.dimensions();

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

    let mut g_img = ImageBuffer::<Luma<i32>, Vec<i32>>::new(width, height);
    let mut theta_img = ImageBuffer::<Luma<f32>, Vec<f32>>::new(width, height);

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let gx = apply_filter(&img, &kernel_x, x, y);
            let gy = apply_filter(&img, &kernel_y, x, y);

            let g = (gx * gx + gy * gy).isqrt();
            let theta = (gy as f32).atan2(gx as f32);

            g_img.put_pixel(x, y, Luma([g]));
            theta_img.put_pixel(x, y, Luma([theta]));
        }
    }

    (g_img, theta_img)
}

enum Direction {
    Horizontal,
    Diagonal45,
    Vertical,
    Diagonal135,
}

fn nms(g_img: &IntImage, theta_img: &FloatImage) -> IntImage {
    let (width, height) = g_img.dimensions();

    let mut result_img = ImageBuffer::<Luma<i32>, Vec<i32>>::new(width, height);

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let theta = theta_img.get_pixel(x, y)[0];
            let norm_t = (theta + PI) % PI;

            let direction = if norm_t >= PI / 8.0 && norm_t < 3.0 * PI / 8.0 {
                Direction::Diagonal45
            } else if norm_t >= 3.0 * PI / 8.0 && norm_t < 5.0 * PI / 8.0 {
                Direction::Horizontal
            } else if norm_t >= 5.0 * PI / 8.0 && norm_t < 7.0 * PI / 8.0 {
                Direction::Diagonal135
            } else {
                Direction::Vertical
            };

            let neighbors = match direction {
                Direction::Horizontal => {
                    vec![g_img.get_pixel(x - 1, y)[0], g_img.get_pixel(x + 1, y)[0]]
                }
                Direction::Diagonal45 => {
                    vec![
                        g_img.get_pixel(x + 1, y + 1)[0],
                        g_img.get_pixel(x - 1, y - 1)[0],
                    ]
                }
                Direction::Vertical => {
                    vec![g_img.get_pixel(x, y - 1)[0], g_img.get_pixel(x, y + 1)[0]]
                }
                Direction::Diagonal135 => {
                    vec![
                        g_img.get_pixel(x - 1, y + 1)[0],
                        g_img.get_pixel(x + 1, y - 1)[0],
                    ]
                }
            };

            let g = g_img.get_pixel(x, y)[0];
            let g = if g >= neighbors.into_iter().max().unwrap() {
                g
            } else {
                0
            };

            result_img.put_pixel(x, y, Luma([g]));
        }
    }

    result_img
}

fn double_threshold(img: &IntImage, high_threshold: i32, low_threshold: i32) -> GrayImage {
    // 強エッジ:255 弱エッジ:127 非エッジ:0
    let (width, height) = img.dimensions();

    let mut result_img = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let g = img.get_pixel(x, y)[0];

            let g = if g >= high_threshold {
                255
            } else if g >= low_threshold {
                127
            } else {
                0
            };

            result_img.put_pixel(x, y, Luma([g]));
        }
    }

    result_img
}

struct Position {
    x: i32,
    y: i32,
}

fn dfs(img: &mut GrayImage, mut stack: Vec<Position>) {
    let (width, height) = img.dimensions();

    loop {
        let pos = match stack.pop() {
            Some(pos) => pos,
            None => return,
        };

        for ny in (pos.y - 1)..(pos.y + 2) {
            for nx in (pos.x - 1)..(pos.x + 2) {
                // 範囲外ならスキップ
                if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                    continue;
                }

                let pixel = img.get_pixel(nx as u32, ny as u32)[0];

                // 弱いエッジでないならスキップ
                if pixel != 127 {
                    continue;
                }

                img.put_pixel(nx as u32, ny as u32, Luma([255]));
                stack.push(Position { x: nx, y: ny });
            }
        }
    }
}

fn hysteresis_threshold(mut img: GrayImage) -> GrayImage {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];

            if pixel == 255 {
                let mut stack = Vec::<Position>::new();
                stack.push(Position {
                    x: x as i32,
                    y: y as i32,
                });
                dfs(&mut img, stack);
            }
        }
    }

    // 残った弱エッジを消す
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];

            if pixel == 127 {
                img.put_pixel(x, y, Luma([0]));
            }
        }
    }

    img
}

fn main() {
    let ori_img = ImageReader::open("images/cat_gray.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();

    let (width, height) = ori_img.dimensions();

    println!("width: {width}, height: {height}");

    let blur_img = gaussian_filter(&ori_img);
    let (g_img, theta_img) = sobel_filter(&blur_img);

    let edge_img = nms(&g_img, &theta_img);
    let edge_img = double_threshold(&edge_img, 100, 50);

    let result_img = hysteresis_threshold(edge_img);

    result_img.save("images/out.png").unwrap();
}
