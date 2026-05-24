// darkS3c: MASTERPIECES (expanded for readability)

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

fn palette(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([5, 2, 15]); }
    let p = t as f64 / mi as f64 * 4.0 * 3.14159;
    let r = ((p.sin() * 80.0 + 175.0).max(20.0)) as u8;
    let g = ((p.sin() * 60.0 + 140.0).max(10.0)) as u8;
    let b = ((p.cos() * 50.0 + 170.0).max(30.0)) as u8;
    Rgb([r, g, b])
}

fn render_and_save(cr: f64, ci: f64, filename: &str) {
    let w = 900u32;
    let h = 900u32;
    let mi = 300u32;
    let bound = 1.6_f64;
    let sc = 2.0 * bound / w as f64;
    let mut img: RgbImage = ImageBuffer::new(w, h);
    
    for py in 0..h {
        let zi = -bound + (py as f64) * sc;
        for px in 0..w {
            let zr = -bound + (px as f64) * sc;
            let t = julia(zr, zi, cr, ci, mi);
            img.put_pixel(px, py, palette(t, mi));
        }
    }
    img.save(filename).unwrap();
}

fn main() {
    println!("=== MASTERPIECES ===");
    
    render_and_save(-0.4, 0.6, "douady_master.png");
    println!("Douady saved");
    
    render_and_save(-0.123, 0.745123, "spiral_master.png");
    println!("Spiral saved");
    
    render_and_save(-0.8, 0.156, "dragon_master.png");
    println!("Dragon saved");
    
    render_and_save(-0.1, 0.651, "curious_master.png");
    println!("Curious saved");
    
    println!("\n4 MASTERPIECES in the gallery!");
}