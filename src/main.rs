// ANIMATION: Julia morph through parameter space
// 30 frames from spiral region to dragon region

use image::{ImageBuffer, Rgb, RgbImage};

fn iterate(mut zr: f64, mut zi: f64, cr: f64, ci: f64, mi: u32) -> u32 {
    for i in 0..mi {
        if zr*zr + zi*zi > 4.0 { return i; }
        let nz = zr*zr - zi*zi + cr;
        zi = 2.0*zr*zi + ci;
        zr = nz;
    }
    mi
}

fn color(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([3, 1, 8]); }
    let p = t as f64 / mi as f64;
    let w = p * 3.0 * 3.14159;
    let r = ((w.cos() * 120.0 * (1.0-p) + 220.0 * p) as u8).max(15).min(255);
    let g = ((w.sin() * 90.0 * (1.0-p) + 180.0 * p.sqrt()) as u8).max(10).min(245);
    let b = ((w.cos() * 70.0 * (1.0-p) + 240.0 * p) as u8).max(40).min(255);
    Rgb([r, g, b])
}

fn main() {
    println!("=== JULIA MORPH 30 FRAMES ===");
    
    let (sz, mi, frames) = (600u32, 300u32, 30u32);
    let b = 1.6_f64;
    let sc = 2.0 * b / sz as f64;
    
    // Path from -0.1+0.65i toward -0.75+0.1i
    for f in 0..frames {
        let t = f as f64 / (frames - 1) as f64;
        let cr = -0.1 - t * 0.65;
        let ci = 0.65 - t * 0.55;
        
        let mut img: RgbImage = ImageBuffer::new(sz, sz);
        
        for py in 0..sz {
            let zi = -b + py as f64 * sc;
            for px in 0..sz {
                let zr = -b + px as f64 * sc;
                let iter = iterate(zr, zi, cr, ci, mi);
                img.put_pixel(px, py, color(iter, mi));
            }
        }
        
        let name = format!("morph_{:02}.png", f);
        img.save(&name).unwrap();
        println!("Frame {}/{}: c={:.3}+{:.3}i", f+1, frames, cr, ci);
    }
    
    println!("SAVED: 30 morph frames");
}