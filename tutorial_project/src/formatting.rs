//! Formatting demo: shows println! with positional, named, and trait-based formatters.
#![allow(dead_code)]

pub fn demo_formatting() {
    // Basic prints
    println!("Hello, world!");
    println!("Hello, {}!", "Alex");
    println!("Hello, {}, {}!", "Alex", "Tom");

    // Positional arguments
    println!("Hello {0}, {1}, {0} again!", "Alex", "Tom");

    // Named arguments
    println!(
        "Hello {first}, {second}!",
        first = "Alex",
        second = "Tom"
    );

    // Integer formatting traits
    println!("Binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);

    // Debug formatting for collections
    println!("Array {:?}", [1, 2, 3]);
}
