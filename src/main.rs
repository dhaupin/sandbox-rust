// AURORA - green/northern lights palette

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

fn aurora(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([5, 15, 10]); }
    let p = t as f64 / mi as f64;
    let wave = p * 8.0;
    let r = (20.0 * p) as u8;
    let g = ((wave.sin() * 60.0 + 200.0 * p.sqrt()) as u8).min(255);
    let b = ((wave.cos() * 30.0 + 120.0) as u8).max(40);
    Rgb([r, g.min(220), b])
}

fn main() {
    let (sz, mi) = (800u32, 400u32);
    let b = 1.6_f64;
    let sc = 2.0 * b / sz as f64;
    let (cr, ci) = (-0.8, 0.156);
    
    let mut img: RgbImage = ImageBuffer::new(sz, sz);
    for py in 0..sz {
        let zi = -b + py as f64 * sc;
        for px in 0..sz {
            let zr = -b + px as f64 * sc;
            img.put_pixel(px, py, aurora(julia(zr, zi, cr, ci, mi), mi));
        }
    }
    img.save("aurora_dragon.png").unwrap();
    println!("AURORA DRAGON: northern lights Dragon");
}