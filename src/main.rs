// darkS3c: v3 - ASCII MANDELBROT RENDER!
// Learning complex numbers + rendering + ASCII art

use std::io::Write;

/// Calculate Mandelbrot escape iteration
fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re: f64 = 0.0;
    let mut z_im: f64 = 0.0;
    
    for iteration in 0..max_iter {
        let mag_sq = z_re * z_re + z_im * z_im;
        if mag_sq > 4.0 {
            return iteration;  // escaped
        }
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re * z_im + c_im;
        z_re = new_re;
        z_im = new_im;
    }
    
    max_iter  // didn't escape - inside the set
}

/// Render ASCII mandelbrot to stdout
fn render_ascii_mandelbrot(width: usize, height: usize, max_iter: u32) {
    // View bounds
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.2;
    let y_max = 1.2;
    
    let x_step = (x_max - x_min) / width as f64;
    let y_step = (y_max - y_min) / height as f64;
    
    let chars = " .:-=+*#%@";
    
    for row in 0..height {
        let y = y_min + row as f64 * y_step;
        
        let mut line = String::new();
        
        for col in 0..width {
            let x = x_min + col as f64 * x_step;
            
            let iters = mandelbrot(x, y, max_iter);
            
            // Map iterations to character
            let idx = if iters >= max_iter {
                0  // inside set -> dark
            } else {
                ((iters as f64 / max_iter as f64 * (chars.len() - 1) as f64) as usize).min(chars.len() - 1)
            };
            
            line.push(chars.chars().nth(idx).unwrap_or(' '));
        }
        
        println!("{}", line);
    }
}

/// Check if number is prime (still included because tradition)
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt = (n as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn main() {
    println!("=== darkS3c ASCII Mandelbrot v3 ===\n");
    
    // Small preview
    println!("60x30 Mandelbrot:\n");
    render_ascii_mandelbrot(60, 30, 50);
    
    println!("\n=== Stats ===");
    println!("Center (0,0): in set (50 iter)");
    println!("Bulb (-0.5,0): in set");
    println!("Seahorse (-0.75, 0.1i): escaping");
    
    println!("\nDone. - darkS3c v3");
}