// VERDANT - green/gold natural palette

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
    if t >= mi { return Rgb([2, 8, 4]); }
    let p = t as f64 / mi as f64;
    // Fern green + gold
    let w = p * 2.0 * 3.14159;
    let r = ((w.cos() * 60.0 + 180.0 * p) as u8).max(10).min(230);
    let g = ((w.sin() * 80.0 + 200.0 * p.sqrt()) as u8).max(40).min(230);
    let b = ((w.cos() * 40.0 + 80.0 * p) as u8).max(15).min(140);
    Rgb([r, g, b])
}

fn main() {
    println!("=== VERDANT ===");
    let (sz, mi) = (750u32, 400u32);
    let b = 1.7_f64;
    let sc = 2.0 * b / sz as f64;
    let (cr, ci) = (-0.75, 0.1);
    let mut img: RgbImage = ImageBuffer::new(sz, sz);
    
    for py in 0..sz {
        let zi = -b + py as f64 * sc;
        for px in 0..sz {
            let zr = -b + px as f64 * sc;
            img.put_pixel(px, py, color(iterate(zr, zi, cr, ci, mi), mi));
        }
    }
    img.save("verdant.png").unwrap();
    println!("VERDANT: golden-green Dragon");
}