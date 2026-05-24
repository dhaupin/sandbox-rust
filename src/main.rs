// darkS3c: PRETTY JULIA SAMPLER
// Find beautiful Julia sets by random sampling

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

fn render(cr: f64, ci: f64) {
    let w = 500u32;
    let h = 500u32;
    let mi = 80u32;
    let bound = 1.6_f64;
    let sc = 2.0 * bound / w as f64;
    let mut img = ImageBuffer::new(w, h);
    
    for py in 0..h {
        let zi = -bound + py as f64 * sc;
        for px in 0..w {
            let zr = -bound + px as f64 * sc;
            let t = julia(zr, zi, cr, ci, mi);
            let v = if t >= mi { 0u8 } else { (t as f64 * 255.0 / mi as f64) as u8 };
            // Electric palette!
            let r = v;
            let g = if v > 100 { v.wrapping_sub(50) } else { v / 2 };
            let b = v.wrapping_add(150);
            img.put_pixel(px, py, Rgb([r, g, b]));
        }
    }
    img.save(&format!("pretty_{:.3}_{:.3}.png", cr, ci)).unwrap();
    println!("c = {:.3}+{:.3}i", cr, ci);
}

fn main() {
    println!("=== PRETTY JULIA FINDER ===");
    
    let samples = vec![
        // Known beautiful regions
        (-0.4, 0.6),    // Douady rabbit
        (-0.8, 0.156), // Dragon  
        (0.285, 0.01),  // San Marco
        (-0.835, -0.2321), // Siegel
        (-0.4, 0.0),   // Dendrite
        (-0.1, 0.651), // Interesting
        (-0.75, 0.1),  // Seahorse
        (-1.401155, 0.0), // Bug
        (0.0, 0.0),   // Circle
        (-0.123, 0.745123), // Spiral!
        // Random beautiful ones
        (-0.7, 0.27015),
        (-0.2, 0.8),
        (-1.0, 0.0),
        (-0.95, 0.2),
        (0.37, 0.1),
    ];
    
    for (cr, ci) in samples {
        render(cr, ci);
    }
    
    println!("\nDone! Pretties found.");
}