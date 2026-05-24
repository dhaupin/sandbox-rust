// DEEP MANDELBROT: Elephant Valley zoom!

use image::{ImageBuffer, Rgb, RgbImage};

fn mandel(cx: f64, cy: f64, mi: u32) -> u32 {
    let mut zr = 0.0_f64;
    let mut zi = 0.0_f64;
    for i in 0..mi {
        if zr*zr + zi*zi > 4.0 { return i; }
        let nz = zr*zr - zi*zi + cx;
        zi = 2.0*zr*zi + cy;
        zr = nz;
    }
    mi
}

fn palette(t: u32, mi: u32) -> Rgb<u8> {
    if t >= mi { return Rgb([2, 1, 5]); }
    let p = t as f64 / mi as f64;
    let w = p * 3.0 * 3.14159;
    let r = ((w.cos() * 120.0 * (1.0-p) + 200.0 * p) as u8).max(10).min(255);
    let g = ((w.sin() * 90.0 * (1.0-p) + 160.0 * p.sqrt()) as u8).max(8).min(245);
    let b = ((w.cos() * 70.0 * (1.0-p) + 220.0 * p) as u8).max(30).min(255);
    Rgb([r, g, b])
}

fn main() {
    println!("=== DEEP ELEPHANT VALLEY ===");
    
    let (sz, mi) = (700u32, 400u32);
    
    // Deep zoom into elephant valley c ≈ -1.764 + 0.027i
    let (cx0, cy0) = (-1.764, 0.027);
    let zoom = 0.015;
    let sc = 2.0 * zoom / sz as f64;
    
    let mut img: RgbImage = ImageBuffer::new(sz, sz);
    
    for py in 0..sz {
        let cy = cy0 + py as f64 * sc - zoom;
        for px in 0..sz {
            let cx = cx0 + px as f64 * sc - zoom;
            let t = mandel(cx, cy, mi);
            img.put_pixel(px, py, palette(t, mi));
        }
        if py % 150 == 0 { println!("Line {}/{}", py, sz); }
    }
    
    img.save("deep_elephant.png").unwrap();
    println!("SAVED: deep_elephant.png - Elephant Valley zoom");
}