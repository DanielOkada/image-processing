// 大津の二値化

use image::{GrayImage, ImageReader, Luma};

fn calc_histogram(img: &GrayImage) -> [i64; 256] {
    let mut histogram = [0i64; 256];

    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];

            histogram[pixel as usize] += 1;
        }
    }

    return histogram;
}

fn calc_class_between_variance(histogram: &[i64; 256], t: usize) -> f64 {
    // σ²_between = w₀ × w₁ × (μ₀ - μ₁)²

    let total = histogram.iter().sum::<i64>();
    let dark_pixels = &histogram[..=t];
    let dark_pixel_count = dark_pixels.iter().sum::<i64>();
    let w0 = dark_pixel_count as f64 / total as f64;
    let w1 = 1.0 - w0;

    let weighted_sum: i64 = (0..=t as i64)
        .map(|pixel_value| pixel_value * histogram[pixel_value as usize])
        .sum();
    let average0 = weighted_sum as f64 / dark_pixel_count as f64;

    let weighted_sum: i64 = ((t as i64 + 1)..256)
        .map(|pixel_value| pixel_value * histogram[pixel_value as usize])
        .sum();
    let average1 = weighted_sum as f64 / (total - dark_pixel_count) as f64;

    return w0 * w1 * (average0 - average1).powi(2);
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

    let histogram = calc_histogram(&img);

    let mut variances = [0f64; 256];
    for t in 0..255 {
        let between_variance = calc_class_between_variance(&histogram, t);

        variances[t as usize] = between_variance;
    }

    let (best_t, _best_variance) = variances
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("threshold: {}", best_t);

    for y in 0..height {
        for x in 0..width {
            let new_pixel = if img.get_pixel(x, y)[0] > best_t as u8 {
                255
            } else {
                0
            };

            img.put_pixel(x, y, Luma([new_pixel as u8]));
        }
    }

    img.save("images/out.png").unwrap();
}
