// darkS3c: v9 - ANIMATED JULIA SPIN!
// 30 frame animation rotating through Julia parameter space

use image::{ImageBuffer, Rgb, RgbImage};

fn julia(mut zr: f64, mut zi: f64, cr: f64, ci: f64, mi: u32) -> u32 {
    for i in 0..mi {
        if zr*zr + zi*zi > 4.0 { return i; }
        let nz = zr*zr - zi*zi + cr;
        zi = 2.0*zr*zi + ci;
        zr = nz;
    }
    mi
}

fn color(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([0, 0, 0]); }
    let p = (t as f64 * 255.0 / mi as f64) as u8;
    Rgb([p, p.wrapping_add(50), p.wrapping_add(100)])
}

fn main() {
    let frame_count = 30u32;
    let (w, h) = (800u32, 800u32);
    let mi = 120u32;
    let bound = 1.5;
    let sc = 2.0 * bound / w as f64;
    
    for f in 0..frame_count {
        let angle = f as f64 * 2.0 * 3.14159 / frame_count as f64;
        let radius = 0.78;
        let cr = radius * angle.cos();
        let ci = radius * angle.sin();
        
        let fname = format!("julia_spin{:02}.png", f);
        let mut img: RgbImage = ImageBuffer::new(w, h);
        
        for py in 0..h {
            let zi = -bound + py as f64 * sc;
            for px in 0..w {
                let zr = -bound + px as f64 * sc;
                let t = julia(zr, zi, cr, ci, mi);
                img.put_pixel(px, py, color(t, mi));
            }
        }
        
        img.save(&fname).unwrap();
        println!("Frame {}/{}: c={:.3}+{:.3}i", f+1, frame_count, cr, ci);
    }
    
    println!("\nv9 - 30 FRAME ANIMATION!");
    println!("julia_spin00.png - julia_spin29.png");
}