// FINAL STAND - one beautiful Julia at extreme detail

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
    if t >= mi { return Rgb([2, 1, 8]); }
    let p = t as f64 / mi as f64;
    let wave = p * 16.0 * 3.141592;
    let r = ((wave.cos() * 100.0 + 210.0) * (1.0-p*0.3)) as u8;
    let g = ((wave.sin() * 80.0 + 165.0) * (1.0-p*0.4)) as u8;
    let b = ((wave.cos() * 60.0 + 230.0) * (1.0-p*0.2)) as u8;
    Rgb([r.max(15), g.max(10), b.max(25)])
}

fn main() {
    let sz = 1000u32;
    let mi = 500u32;
    let b = 1.6_f64;
    let sc = 2.0 * b / sz as f64;
    let (cr, ci) = (-0.4, 0.6);
    
    let mut img = ImageBuffer::new(sz, sz);
    for py in 0..sz {
        let zi = -b + py as f64 * sc;
        for px in 0..sz {
            let zr = -b + px as f64 * sc;
            let t = julia(zr, zi, cr, ci, mi);
            img.put_pixel(px, py, color(t, mi));
        }
    }
    img.save("final_stand.png").unwrap();
    println!("FINAL STAND: Douady @ 1000x1000, iters=500");
}