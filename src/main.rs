// NEBULA - ethereal cloud-like Julia coloring
// Using escape-time + smooth iteration technique

use image::{ImageBuffer, Rgb, RgbImage};

fn iterate(mut zr: f64, mut zi: f64, cr: f64, ci: f64, mi: u32) -> (u32, f64) {
    for i in 0..mi {
        let d = (zr*zr + zi*zi).sqrt();
        if d > 2.0 { return (i, d - 2.0); }
        let nz = zr*zr - zi*zi + cr;
        zi = 2.0*zr*zi + ci;
        zr = nz;
    }
    (mi, 0.0)
}

fn nebula(iter: u32, esc: f64, mi: u32) -> Rgb<u8> {
    if iter >= mi { return Rgb([15, 8, 30]); }
    
    let s = iter as f64 + 1.0 - esc.ln() / 0.693147;  // smooth
    let t = s / mi as f64;
    let cycles = t * 12.0;
    
    // Lavender, aqua, rose nebula tones
    let r = ((cycles.cos() * 90.0 + 200.0 * (1.0-t))).max(20.0).min(255.0) as u8;
    let g = ((cycles.sin() * 70.0 + 160.0 * (1.0-t))).max(25.0).min(255.0) as u8;
    let b = ((cycles.cos() * 50.0 + 220.0 * (1.0-t))).max(50.0).min(255.0) as u8;
    
    Rgb([r, g, b])
}

fn main() {
    println!("=== NEBULA SERIES ===\n");
    
    let sz = 800u32;
    let mi = 350u32;
    let b = 1.7_f64;
    let sc = 2.0 * b / sz as f64;
    
    let constants = vec![
        (-0.4, 0.6),
        (-0.1, 0.651),
        (-0.75, 0.1),
    ];
    
    for (c1, c2) in constants {
        let fname = format!("nebula_{:.1}_{:.1}.png", c1, c2);
        let mut img: RgbImage = ImageBuffer::new(sz, sz);
        
        for py in 0..sz {
            let zi = -b + py as f64 * sc;
            for px in 0..sz {
                let zr = -b + px as f64 * sc;
                let (it, es) = iterate(zr, zi, c1, c2, mi);
                img.put_pixel(px, py, nebula(it, es, mi));
            }
        }
        img.save(&fname).unwrap();
        println!("NEBULA: {}", fname);
    }
    
    println!("\n=== NEBULA COMPLETE ===");
}