// darkS3c: v7 - DEEP ZOOM!
// Zoom into Seahorse valley - find the mini-brot

use image::{ImageBuffer, Rgb, RgbImage};

fn iter_point(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;
    
    for i in 0..max_iter {
        let m = z_re * z_re + z_im * z_im;
        if m > 4.0 { return i; }
        let nr = z_re * z_re - z_im * z_im + c_re;
        let ni = 2.0 * z_re * z_im + c_im;
        z_re = nr;
        z_im = ni;
    }
    max_iter
}

fn color(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([0, 0, 0]); }
    let s = (t as f64 / mi as f64 * 255.0) as u8;
    Rgb([s, (s.wrapping_add(40)), (s.wrapping_add(80))])
}

fn main() {
    let (w, h) = (1600u32, 1200u32);
    let mi = 300u32;
    
    // Deep zoom into Seahorse Valley (-0.7453, 0.1127i)
    let cx = -0.7453;
    let cy = 0.1127;
    let zoom = 300.0;  // How far to zoom
    
    let scale = 4.0 / zoom / w.min(h) as f64;
    
    let mut img: RgbImage = ImageBuffer::new(w, h);
    
    println!("Deep Zoom x{} at ({}, {})...", zoom, cx, cy);
    
    for py in 0..h {
        let y = cy + (py as f64 - h as f64 / 2.0) * scale;
        for px in 0..w {
            let x = cx + (px as f64 - w as f64 / 2.0) * scale;
            let t = iter_point(x, y, mi);
            img.put_pixel(px, py, color(t, mi));
        }
        if py % 200 == 0 { println!("Row {}/{}", py, h); }
    }
    
    img.save("deep_zoom.png").expect("Save failed!");
    println!("DEEP ZOOM v7: deep_zoom.png {}x{}", w, h);
}