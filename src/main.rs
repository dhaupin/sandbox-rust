// LAST LIGHT - sunset palette

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
    if t >= mi { return Rgb([10, 5, 20]); }
    let p = t as f64 / mi as f64;
    let r = (p * 255.0 * 1.2) as u8;
    let g = (p.sin() * 100.0 + p * 150.0) as u8;
    let b = (50.0 - p * 30.0) as u8;
    Rgb([r.min(255), g.min(220).max(20), b.max(10)])
}

fn main() {
    let sz = 600u32;
    let mi = 250u32;
    let b = 1.5_f64;
    let sc = 2.0 * b / sz as f64;
    let (cr, ci) = (-0.1, 0.651);
    
    let mut img = ImageBuffer::new(sz, sz);
    for py in 0..sz {
        let zi = -b + py as f64 * sc;
        for px in 0..sz {
            let zr = -b + px as f64 * sc;
            let t = julia(zr, zi, cr, ci, mi);
            img.put_pixel(px, py, color(t, mi));
        }
    }
    img.save("last_light.png").unwrap();
    println!("LAST LIGHT: sunset Julia");
}