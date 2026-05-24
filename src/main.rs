// darkS3c: GRAND FINALE
// The most beautiful Julia set we'll make tonight!

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

// The ULTIMATE coral-silver-midnight palette
fn ultimate(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([8, 3, 18]); }
    let p = t as f64 / mi as f64;
    
    // Smooth gradient with coral, silver, midnight blue
    let phase = p * 6.28318530718 * 2.0;
    
    let r = ((phase.cos() * 120.0 + 180.0 * p) / 1.5).max(15.0).min(255.0) as u8;
    let g = ((phase.sin() * 80.0 + 140.0 * p) / 1.5).max(10.0).min(255.0) as u8;
    let b = ((phase.cos() * 60.0 + 200.0 * p) / 1.5).max(40.0).min(255.0) as u8;
    
    Rgb([r, g, b])
}

fn main() {
    println!("=== GRAND FINALE ===\n");
    
    let size = 1200u32;
    let mi = 400u32;
    let bound = 1.65_f64;
    let sc = 2.0 * bound / size as f64;
    
    // Most aesthetically interesting Julia constants
    let targets = vec![
        (-0.4, 0.6, "douady_grand"),
        (-0.123, 0.745123, "spiral_grand"),
        (-0.8, 0.156, "dragon_grand"),
    ];
    
    for (cr, ci, name) in targets {
        let mut img: RgbImage = ImageBuffer::new(size, size);
        
        for py in 0..size {
            let zi = -bound + py as f64 * sc;
            for px in 0..size {
                let zr = -bound + px as f64 * sc;
                let t = julia(zr, zi, cr, ci, mi);
                img.put_pixel(px, py, ultimate(t, mi));
            }
            if py % 200 == 0 { println!("{}_: row {}", name, py); }
        }
        
        img.save(&format!("{}.png", name)).unwrap();
        println!("SAVED: {}.png", name);
    }
    
    println!("\n=== GRAND FINALE COMPLETE ===");
    println!("darkS3c signing off... for now.");
}