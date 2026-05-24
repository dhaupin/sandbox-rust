// RANDOM EXPLORER - Find hidden Julia gems!

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
    if t >= mi { return Rgb([3, 1, 8]); }
    let p = t as f64 / mi as f64;
    let w = p * 3.0 * 3.14159;
    let r = ((w.cos() * 120.0 * (1.0-p) + 220.0 * p) as u8).max(15).min(255);
    let g = ((w.sin() * 90.0 * (1.0-p) + 180.0 * p.sqrt()) as u8).max(10).min(245);
    let b = ((w.cos() * 70.0 * (1.0-p) + 240.0 * p) as u8).max(40).min(255);
    Rgb([r, g, b])
}

fn main() {
    // Interesting Julia constants to explore
    let gems = vec![
        (-0.7, 0.27015, "gemma"),     
        (-0.835, -0.2321, "gemmab"),
        (-0.4, 0.0, "gemmac"),        
        (0.285, 0.01, "gemmad"),      
        (-0.1, 0.651, "gemmae"),      
    ];
    
    let (sz, mi) = (500u32, 250u32);
    let b = 1.6_f64;
    let sc = 2.0 * b / sz as f64;
    
    for (cr, ci, name) in gems {
        let mut img: RgbImage = ImageBuffer::new(sz, sz);
        
        for py in 0..sz {
            let zi = -b + py as f64 * sc;
            for px in 0..sz {
                let zr = -b + px as f64 * sc;
                let iter = iterate(zr, zi, cr, ci, mi);
                img.put_pixel(px, py, color(iter, mi));
            }
        }
        
        img.save(&format!("{}.png", name)).unwrap();
        println!("GEM {}: c={:.3}+{:.3}i", name, cr, ci);
    }
    
    println!("DONE: 5 gem explorations");
}