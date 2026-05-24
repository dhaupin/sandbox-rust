// darkS3c: v5 - JULIA SETS!
// Different c values = different shapes

use image::{ImageBuffer, Rgb, RgbImage};

fn iterate(z_re_init: f64, z_im_init: f64, c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = z_re_init;
    let mut z_im = z_im_init;
    
    for iteration in 0..max_iter {
        let mag_sq = z_re * z_re + z_im * z_im;
        if mag_sq > 4.0 { return iteration; }
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re * z_im + c_im;
        z_re = new_re;
        z_im = new_im;
    }
    max_iter
}

fn color(t: f64, max_iter: u32) -> Rgb<u8> {
    if t >= max_iter as f64 { return Rgb([0, 0, 0]); }
    let cycle = t * 4.0;
    Rgb([
        ((cycle.sin() * 127.0 + 128.0) as u8),
        ((cycle.sin() * 127.0 + 128.0) as u8).wrapping_add(40),
        ((cycle.sin() * 127.0 + 128.0) as u8).wrapping_add(80),
    ])
}

fn render_julia(c_re: f64, c_im: f64, filename: &str) {
    let w = 600u32;
    let h = 600u32;
    let max_iter = 100u32;
    let bounds = 1.5;
    
    let scale = bounds * 2.0 / w as f64;
    let mut img: RgbImage = ImageBuffer::new(w, h);
    
    for py in 0..h {
        for px in 0..w {
            let z_re = -bounds + px as f64 * scale;
            let z_im = -bounds + py as f64 * scale;
            let t = iterate(z_re, z_im, c_re, c_im, max_iter);
            img.put_pixel(px, py, color(t as f64, max_iter));
        }
    }
    img.save(filename).expect("Save failed!");
    println!("Saved: {} (c={}+{}i)", filename, c_re, c_im);
}

fn main() {
    println!("=== darkS3c Julia Collection ===\n");
    
    // Famous Julia constants
    render_julia(-0.4, 0.6, "julia1_douady.png");    // Douady rabbit
    render_julia(-0.8, 0.156, "julia2_dragon.png");  // Dragon
    render_julia(0.285, 0.01, "julia3.png");         // San Marco
    render_julia(-0.835, -0.2321, "julia4.png");      // Siegel disk
    render_julia(-0.4, 0.0, "julia5.png");          // Dendrite
    
    // Animated spiral
    let theta = 0.0;
    for i in 0..6 {
        let angle = theta + i as f64 * 1.0472;  // 60 degree steps
        let c = 0.7885 * angle.cos();
        let ci = 0.7885 * angle.sin();
        render_julia(c, ci, &format!("julia_anim{:02}.png", i));
    }
    
    println!("\n7 Julia sets rendered! darkS3c v5");
}