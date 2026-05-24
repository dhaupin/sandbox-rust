// CHAOS BOUNDARY - ultra-high iterations reveal fine tendrils

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
    if t >= mi { return Rgb([1, 0, 3]); }
    let p = t as f64 / mi as f64;
    let w = p * 3.0 * 3.14159;
    let fade = 1.0 - p;
    let r = ((w.cos() * 120.0 * fade + 220.0 * p) as u8).max(8).min(255);
    let g = ((w.sin() * 90.0 * fade + 180.0 * p.sqrt()) as u8).max(5).min(245);
    let b = ((w.cos() * 70.0 * fade + 240.0 * p) as u8).max(25).min(255);
    Rgb([r, g, b])
}

fn main() {
    println!("=== CHAOS BOUNDARY ===");
    let (sz, mi) = (450u32, 500u32);
    let b = 1.65_f64;
    let sc = 2.0 * b / sz as f64;
    let (cr, ci) = (-0.4, 0.6);
    let mut img: RgbImage = ImageBuffer::new(sz, sz);
    
    for py in 0..sz {
        let zi = -b + py as f64 * sc;
        for px in 0..sz {
            let zr = -b + px as f64 * sc;
            let it = iterate(zr, zi, cr, ci, mi);
            img.put_pixel(px, py, color(it, mi));
        }
    }
    img.save("chaos_boundary.png").unwrap();
    println!("SAVED: chaos_boundary 500 iter");
}