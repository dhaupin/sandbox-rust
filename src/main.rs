// MIDNIGHT OCEAN - deep teal/blue palette

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
    if t >= mi { return Rgb([0, 8, 25]); }
    let p = t as f64 / mi as f64;
    let wave = p * 10.0;
    let r = ((wave.cos() * 50.0 + 30.0 * p) as u8);
    let g = ((wave.sin() * 70.0 + 100.0 * p) as u8).max(40);
    let b = ((wave.cos() * 40.0 + 180.0 * p) as u8).max(80);
    Rgb([r, g, b])
}

fn main() {
    let (sz, mi) = (750u32, 350u32);
    let b = 1.6_f64;
    let sc = 2.0 * b / sz as f64;
    let (cr, ci) = (-0.4, 0.6);
    
    let mut img: RgbImage = ImageBuffer::new(sz, sz);
    for py in 0..sz {
        let zi = -b + py as f64 * sc;
        for px in 0..sz {
            let zr = -b + px as f64 * sc;
            img.put_pixel(px, py, color(julia(zr, zi, cr, ci, mi), mi));
        }
    }
    img.save("midnight_ocean.png").unwrap();
    println!("MIDNIGHT OCEAN: teal deep Julia");
}
