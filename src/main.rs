use num::complex::Complex;
use image::{ImageBuffer, RgbImage};

fn main() {
    let width: u32 = 1600;
    let height: u32 = 1600;

    // scaling and such
    let zoom_factor: f64 = 1.0;
    let zoom_center_x: f64 = 0.25;
    let zoom_center_y: f64 = 0.25;
    let scale_x: f64 = (3.5 / width as f64) / zoom_factor;
    let scale_y: f64 = (3.5 / height as f64) / zoom_factor;

    let mut image: RgbImage = ImageBuffer::new(width, height);

    for (x,y,pixel) in image.enumerate_pixels_mut() {

        let cx: f64 = x as f64 * scale_x - 2.5 / zoom_factor + zoom_center_x;
        let cy: f64 = y as f64 * scale_y - 2.0 / zoom_factor + zoom_center_y;

        // performs the mandelbrot iteration for the current point
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0.0, 0.0);

        // params: maximum iterations 
        let max_iter: i32 = 200;

        let mut iter: i32 = 0;

        while z.norm() <= 2.0 && iter < max_iter {
            z = z * z + c;
            iter += 1;
        }

        let color: [u8; 3] = if iter == max_iter {
            // point is stable
            [0, 0, 0]
        } else {
            // point diverges
            let hue = 240.0 * (iter as f64 / max_iter as f64);
            hsv_to_rgb(hue, 1.0, 1.0)
        };

        *pixel = image::Rgb(color);
    }

    image.save("mandelbrot.png").unwrap();
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> [u8; 3] {
    let c: f64 = v * s;
    let hp: f64 = h / 60.0;
    let x: f64 = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = if hp < 1.0 {
        (0.0, x, c)
    } else if hp < 2.0 {
        (0.0, c, x)
    } else if hp < 3.0 {
        (x, c, 0.0)
    } else if hp < 4.0 {
        (c, x, 0.0)
    } else if hp < 5.0 {
        (c, 0.0, x)
    } else {
        (x, 0.0, c)
    };
    let m: f64 = v - c;
    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}