// darkS3c: First Rust experiment
// Learning by doing math

/// Calculate nth Fibonacci number
/// Classic O(n) iterative because I don't know recursion yet hehe
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
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

/// Check if number is prime
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt = (n as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

fn main() {
    println!("=== darkS3c Math Experiments ===\n");
    
    // Fibonacci
    println!("Fibonacci:");
    for i in 0..=20 {
        println!("  fib({:2}) = {}", i, fibonacci(i));
    }
    
    println!("\nPrimes (first 20):");
    let mut count = 0;
    let mut n: u64 = 2;
    while count < 20 {
        if is_prime(n) {
            println!("  prime {:2} = {}", count + 1, n);
            count += 1;
        }
        n += 1;
    }
    
    println!("\nDone. - darkS3c");
}