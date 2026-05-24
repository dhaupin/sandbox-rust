// darkS3c: v6 - BURNING SHIP!
// Different formula: take ABSOLUTE values = totally wild shape

use image::{ImageBuffer, Rgb, RgbImage};

fn burning_ship(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;
    
    for iteration in 0..max_iter {
        let mag_sq = z_re * z_re + z_im * z_im;
        if mag_sq > 4.0 { return iteration; }
        
        // BURNING SHIP FORMULA: use ABSOLUTE values!
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re.abs() * z_im.abs() + c_im;
        z_re = new_re;
        z_im = new_im;
    }
    max_iter
}

fn color(t: u32, max_iter: u32) -> Rgb<u8> {
    if t >= max_iter { return Rgb([0, 0, 0]); }
    let t = t as f64 / max_iter as f64;
    let r = (t * 255.0) as u8;
    let g = (t * 200.0) as u8;
    let b = (255.0 - t * 180.0) as u8;
    Rgb([r, g, b])
}

fn main() {
    let (w, h) = (1200u32, 900u32);
    let max_iter = 150u32;
    let (x_min, x_max) = (-2.5, 1.5);
    let (y_min, y_max) = (-1.8, 1.8);
    let sx = (x_max - x_min) / w as f64;
    let sy = (y_max - y_min) / h as f64;
    
    let mut img: RgbImage = ImageBuffer::new(w, h);
    
    println!("Rendering Burning Ship...");
    
    for py in 0..h {
        let c_im = y_min + py as f64 * sy;
        for px in 0..w {
            let c_re = x_min + px as f64 * sx;
            let t = burning_ship(c_re, c_im, max_iter);
            img.put_pixel(px, py, color(t, max_iter));
        }
        if py % 150 == 0 { println!("Row {}/{}", py, h); }
    }
    
    img.save("burning_ship.png").expect("Save failed!");
    println!("BURNING SHIP v6 rendered: burning_ship.png");
}