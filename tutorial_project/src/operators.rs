//! Constants and operators demo.
#![allow(dead_code)]

pub const URL: &str = "https://www.google.com/"; // Type mandatory; no shadowing for const

pub fn demo_operators() {
    // Show bitwise operations on small integers
    let a: u8 = 0b1010; // 10
    let b: u8 = 0b0110; // 6
    println!("a & b = {:04b}", a & b); // AND  -> 0010
    println!("a | b = {:04b}", a | b); // OR   -> 1110
    println!("a ^ b = {:04b}", a ^ b); // XOR  -> 1100
    println!("!a    = {:04b}", !a & 0b1111); // NOT -> 0101 (mask to 4 bits for display)
    println!("a << 1 = {:04b}", a << 1); // left shift
    println!("a >> 1 = {:04b}", a >> 1); // right shift

    println!("Const URL = {}", URL);
}
