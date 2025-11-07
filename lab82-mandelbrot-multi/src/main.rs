use hsv_to_rgb::hsv_to_rgb;
use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let image_width: u32 = 1920;
    let image_height: u32 = 1080;
    let max_iterations: u32 = 1000;

    let mut imgbuf = ImageBuffer::new(image_width, image_height);

    let scale = 1.0;
    let x_min: f64 = scale*-2.0;
    let x_max: f64 = scale*1.0;
    let y_min: f64 = scale*-1.0;
    let y_max: f64 = scale*1.0;

    let start = Instant::now();

    // Parallel pixel calculation using Rayon.
    // Compute pixels in parallel into a Vec<(x, y, pixel)> then write sequentially to the ImageBuffer.
    let width = image_width as usize;
    let height = image_height as usize;
    let total_pixels = width.saturating_mul(height);

    // Precompute scale factors (use -1.0 to map edges correctly)
    let denom_x = if width > 1 { (width - 1) as f64 } else { 1.0 };
    let denom_y = if height > 1 { (height - 1) as f64 } else { 1.0 };
    let scale_x = (x_max - x_min) / denom_x;
    let scale_y = (y_max - y_min) / denom_y;

    let pixels: Vec<(u32, u32, Rgb<u8>)> = (0..total_pixels)
        .into_par_iter()
        .map(|i| {
            let x_idx = i % width;
            let y_idx = i / width;
            let x = x_idx as u32;
            let y = y_idx as u32;

            // Map pixel -> complex plane
            let c_real = x_min + (x_idx as f64) * scale_x;
            let c_imag = y_min + (y_idx as f64) * scale_y;
            let c = Complex::new(c_real, c_imag);

            let mut z = Complex::new(0.0, 0.0);
            let mut iterations = 0;
            while z.norm_sqr() <= 4.0 && iterations < max_iterations {
                z = z * z + c;
                iterations += 1;
            }

            let pixel = if iterations == max_iterations {
                Rgb([0, 0, 0])
            } else {
                let t = iterations as f32 / max_iterations as f32;
                let hue = 360.0 * ((t * 3.0) % 3.0);
                // use full saturation for color output
                let saturation = 1.0;
                let value = if t < 0.5 { 0.5 + t } else { 1.0 };
                hsv_to_rgb(hue, saturation, value)
            };

            (x, y, pixel)
        })
        .collect();

    // Write pixels to image buffer (sequential, ImageBuffer isn't Sync for direct mutation)
    for (x, y, pixel) in pixels {
        imgbuf.put_pixel(x, y, pixel);
    }

    let duration = start.elapsed();
    println!("Rendering time: {:?}", duration);

    std::fs::create_dir_all("./out").unwrap();
    imgbuf.save("./out/mandelbrot_multi.png").unwrap();
    println!("Image saved to ./out/mandelbrot_multi.png");
}
