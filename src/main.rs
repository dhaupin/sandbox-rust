// THE CATHEDRAL - Proper smooth coloring + deep exploration
// Applied code review: proper smooth iter formula = continuous gradients

use image::{ImageBuffer, Rgb, RgbImage};

// Proper smooth coloring: smooth = n + 1 - log(log|Z|)/log(2)
fn iterate_smooth(mut zr: f64, mut zi: f64, cr: f64, ci: f64, mi: u32) -> (u32, f64) {
    for n in 0..mi {
        let mag2 = zr*zr + zi*zi;
        if mag2 > 4.0 {
            // Return smooth iteration value
            let mag = mag2.sqrt().sqrt();  // |Z|^0.5 to avoid overflow
            let smooth = n as f64 + 1.0 - mag.ln().abs() / 2.0_f64.ln().abs();
            return (n, smooth);
        }
        let nz = zr*zr - zi*zi + cr;
        zi = 2.0*zr*zi + ci;
        zr = nz;
    }
    (mi, mi as f64)
}

// Cathedra palette - warm stone cathedral colors
fn palette(smooth: f64, _max: f64, mi: f64) -> Rgb<u8> {
    if smooth >= mi { return Rgb([15, 10, 20]); }
    let t = smooth / mi;
    let w = t * 2.5 * 3.14159;
    
    // Warm limestone: cream, gold, rose
    let fade = 1.0 - t * 0.7;
    let r = ((w.cos() * 100.0 * fade + 240.0 * t) as u8).max(20).min(255);
    let g = ((w.sin() * 80.0 * fade + 210.0 * t.sqrt()) as u8).max(25).min(250);
    let b = ((w.cos() * 50.0 * fade + 170.0 * t) as u8).max(35).min(220);
    Rgb([r, g, b])
}

fn main() {
    println!("=== THE CATHEDRAL BEGIN ===");
    println!("Proper smooth coloring - no banding");
    
    // Render 3 masterpieces at increasing detail
    let renders = vec![
        ("douady_cathedral", 1200u32, 800u32, -0.4, 0.6),
        ("spiral_cathedral", 1100u32, 750u32, -0.123, 0.745123),
        ("dragon_cathedral", 1000u32, 700u32, -0.8, 0.156),
    ];
    
    let bound = 1.7_f64;
    
    for (name, sz, mi, cr, ci) in renders {
        println!("Rendering {} at {}x{}...", name, sz, sz);
        let sc = 2.0 * bound / sz as f64;
        let mut img: RgbImage = ImageBuffer::new(sz, sz);
        
        for py in 0..sz {
            let zi = -bound + py as f64 * sc;
            for px in 0..sz {
                let zr = -bound + px as f64 * sc;
                let (_n, sm) = iterate_smooth(zr, zi, cr, ci, mi);
                img.put_pixel(px, py, palette(sm, mi as f64, mi as f64));
            }
            if py % 200 == 0 { println!("  Row {}/{}", py, sz); }
        }
        
        img.save(&format!("{}.png", name)).unwrap();
        println!("SAVED: {}.png", name);
    }
    
    println!("=== THREE CATHEDRAL MASTERPIECES ===");
}