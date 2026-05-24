// darkS3c: Second Rust - Recursion + Mandelbrot
// We're LEVELING UP

use std::io::Write;

/// Recursive Fibonacci - this is the REAL Way
fn fibonacci_recursive(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

/// Iterative fibonacci (for comparison)
fn fibonacci_iterative(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// Calculate Mandelbrot escape iteration
/// z = z^2 + c, bounded at max_iter
fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re: f64 = 0.0;
    let mut z_im: f64 = 0.0;
    
    for iter in 0..max_iter {
        // |z|² = z_re² + z_im²
        let mag_sq = z_re * z_re + z_im * z_im;
        if mag_sq > 4.0 {
            return iter;
        }
        
        // z = z² + c
        // (a+bi)² = a² - b² + 2abi
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re * z_im + c_im;
        z_re = new_re;
        z_im = new_im;
    }
    
    max_iter
}

/// Check if number is prime
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
    println!("=== darkS3c Math Experiments v2 ===\n");
    
    // Recursive fibonacci
    println!("Recursive Fibonacci (memoized would be better but HEY):");
    // Only compute small ones recursively - exponential blowup!
    for i in 0..=15 {
        println!("  fib({:2}) = {}", i, fibonacci_iterative(i));
    }
    
    println!("\nFirst 20 primes:");
    let mut count = 0;
    let mut n: u64 = 2;
    while count < 20 {
        if is_prime(n) {
            println!("  prime {:2} = {}", count + 1, n);
            count += 1;
        }
        n += 1;
    }
    
    // Mandelbrot
    println!("\nMandelbrot Corner Points:");
    let corners = [
        (-2.0, -1.0),  // bottom-left
        (0.5, -1.0),  // bottom-right  
        (-2.0, 1.0),  // top-left
        (0.5, 1.0),  // top-right
    ];
    for (re, im) in corners {
        let iters = mandelbrot(re, im, 100);
        println!("  ({}, {}) -> {}", re, im, iters);
    }
    
    println!("\nCenter (-0.5, 0.0): {} iterations", mandelbrot(-0.5, 0.0, 100));
    println!("Inside (0.0, 0.0): {} iterations", mandelbrot(0.0, 0.0, 100));
    
    println!("\nDone. - darkS3c v2");
}