// ORBIT TRAP MANDELBROT - super smooth coloring!

use image::{ImageBuffer, Rgb, RgbImage};

fn mandelbrot(cr: f64, ci: f64, mi: u32) -> (u32, f64) {
    let mut zr = 0.0_f64;
    let mut zi = 0.0_f64;
    let mut min_dist = 1000.0_f64;
    
    for i in 0..mi {
        let dist_sq = zr*zr + zi*zi;
        if dist_sq < min_dist * min_dist { min_dist = dist_sq.sqrt(); }
        if dist_sq > 4.0 { return (i, min_dist); }
        let nz = zr*zr - zi*zi + cr;
        zi = 2.0*zr*zi + ci;
        zr = nz;
    }
    (mi, min_dist)
}

fn smooth_color(iter: u32, min_dist: f64, mi: u32) -> Rgb<u8> {
    if iter >= mi { return Rgb([0, 0, 5]); }
    
    // Smooth coloring formula
    let smoothed = iter as f64 + 4.0 - min_dist.ln() / 2.0_f64.ln();
    let t = smoothed / mi as f64;
    
    // Blue-gold ocean palette
    let r = ((t * 280.0).sin() * 80.0 + 130.0).max(10.0) as u8;
    let g = ((t * 200.0).cos() * 70.0 + 100.0).max(5.0) as u8;
    let b = ((t * 320.0).sin() * 100.0 + 180.0).max(40.0) as u8;
    
    Rgb([r, g, b])
}

fn main() {
    println!("=== ORBIT TRAP MANDELBROT ===\n");
    
    let w = 1000u32;
    let h = 800u32;
    let mi = 500u32;
    let (xm, xM) = (-2.0, 0.6);
    let (ym, yM) = (-1.2, 1.2);
    let sx = (xM - xm) / w as f64;
    let sy = (yM - ym) / h as f64;
    
    let mut img = ImageBuffer::new(w, h);
    
    for py in 0..h {
        let ci = ym + py as f64 * sy;
        for px in 0..w {
            let cr = xm + px as f64 * sx;
            let (iter, md) = mandelbrot(cr, ci, mi);
            img.put_pixel(px, py, smooth_color(iter, md, mi));
        }
        if py % 100 == 0 { println!("Line {}/{}", py, h); }
    }
    
    img.save("orbit_trap_mandelbrot.png").unwrap();
    println!("SAVED: orbit_trap_mandelbrot.png - smooth coloring!");
}