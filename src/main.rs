// ELECTRIC SHOCK - glowing neon palatte

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
    let p = t as f64 / mi as f64;
    // HOT NEON: cyan, magenta, electric yellow
    if p < 0.3 { 
        let g = (p / 0.3 * 255.0) as u8;
        Rgb([0, g, g])  // Cyan burst
    } else if p < 0.7 {
        let g = ((p - 0.3) / 0.4 * 255.0) as u8;
        Rgb([g, 0, g])  // Magenta
    } else {
        let g = ((p - 0.7) / 0.3 * 255.0) as u8;
        Rgb([g, g, 0]) // Yellow
    }
}

fn main() {
    let sz = 700u32;
    let mi = 300u32;
    let b = 1.5_f64;
    let sc = 2.0 * b / sz as f64;
    let (cr, ci) = (-0.7, 0.27015);
    
    let mut img = ImageBuffer::new(sz, sz);
    for py in 0..sz {
        let zi = -b + py as f64 * sc;
        for px in 0..sz {
            let zr = -b + px as f64 * sc;
            let t = julia(zr, zi, cr, ci, mi);
            img.put_pixel(px, py, color(t, mi));
        }
    }
    img.save("electric_shock.png").unwrap();
    println!("ELECTRIC SHOCK: neon Julia @ 700x700");
}