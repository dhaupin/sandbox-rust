// darkS3c: v8 - NEON RAINBOW!
// Cycle through HSL hues for psychedelic colors

use image::{ImageBuffer, Rgb, RgbImage};

fn mandelbrot(c_re: f64, c_im: f64, mi: u32) -> u32 {
    let mut zr = 0.0_f64;
    let mut zi = 0.0_f64;
    for i in 0..mi {
        if zr*zr + zi*zi > 4.0 { return i; }
        let nz = zr*zr - zi*zi + c_re;
        zi = 2.0*zr*zi + c_im;
        zr = nz;
    }
    mi
}

// Convert HSV to RGB (hand-rolled, no deps)
fn hsv2rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    
    let (r, g, b) = if h < 60.0 { (c, x, 0.0) }
        else if h < 120.0 { (x, c, 0.0) }
        else if h < 180.0 { (0.0, c, x) }
        else if h < 240.0 { (0.0, x, c) }
        else if h < 300.0 { (x, 0.0, c) }
        else { (c, 0.0, x) };
    
    (((r+m)*255.0) as u8, ((g+m)*255.0) as u8, ((b+m)*255.0) as u8)
}

fn color(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([0, 0, 0]); }
    let hue = (t as f64 / mi as f64 * 360.0 * 4.0) % 360.0;
    let (r, g, b) = hsv2rgb(hue, 0.9, 0.9);
    Rgb([r, g, b])
}

fn main() {
    let (w, h) = (1400u32, 1000u32);
    let mi = 200u32;
    let (xm, xM, ym, yM) = (-2.0, 0.7, -1.15, 1.15);
    let sx = (xM - xm) / w as f64;
    let sy = (yM - ym) / h as f64;
    
    let mut img: RgbImage = ImageBuffer::new(w, h);
    
    println!("NEON rainbow rendering...");
    for py in 0..h {
        let yi = ym + py as f64 * sy;
        for px in 0..w {
            let xi = xm + px as f64 * sx;
            let t = mandelbrot(xi, yi, mi);
            img.put_pixel(px, py, color(t, mi));
        }
    }
    img.save("neon_mandelbrot.png").unwrap();
    println!("v8 - NEON RAINBOW! neon_mandelbrot.png");
}