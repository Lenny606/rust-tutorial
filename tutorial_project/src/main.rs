//! The `main` function serves as the entry point of the program.
//!
//! This example demonstrates how to:
//! - Read user input from the standard input.
//! - Handle the result of the input operation using a `match` statement.
//! - Display the user input or an error message.
//! - Showcase different `println!` formatting options, including positional, named arguments, traits, and debug formatting.
//!
//! # Code Behavior:
//! 1. Initializes a mutable `String` variable to store user input.
//! 2. Asks the user to input text and reads it into the variable.
//! 3. Prints back what the user wrote or an error if the input failed.
//! 4. Demonstrates various `println!` formatting options.
//!
//! # Examples:
//!
//! ```rust
//! // When the program is executed:
//! // User enters: "Hello!"
//! // Output:
//! // Write smthig
//! // You wrote: Hello!
//! // Hello, world!
//! // Hello, Alex!
//! // Hello, Alex, Tom!
//! // Hello Alex, Tom, Alex again!
//! // Hello Alex, Tom!
//! // Binary: 110010, hex: 32, octa: 62
//! // Array [1, 2, 3]
//! ```
//!
//! ### Key Concepts Demonstrated:
//! 1. **Input Handling**:
//!    - Uses `io::stdin().read_line()` to capture user input.
//!    - Handles both successful and failed input scenarios using a `match` statement.
//! 2. **Formated Output**:
//!    - Demonstrates positional and named arguments in formatting strings with `println!`.
//!    - Demonstrates binary, hexadecimal, and octal formatting.
//!    - Prints arrays using `Debug` formatting.
use std::io;

/// The `main` function serves as the entry point of the program.
///
/// # Overview
/// This function demonstrates user input retrieval, error handling, and various formatting capabilities in Rust.
///
/// ## Features
/// - Reads and displays user input.
/// - Demonstrates the usage of the `println!` macro for formatted output.
/// - Covers positional formatting, named arguments, traits, and debug printing.
///
/// ## Functionality
/// 1. **Input Handling**:
///    - Prompts the user to input text.
///    - Reads the user input and stores it in a mutable `String` variable.
///    - Handles possible errors during user input using `match`.
/// 2. **Formatted Output**:
///    - Utilizes `println!` to showcase different formatting styles:
///      - Positional arguments.
///      - Named arguments.
///      - Binary, hex, and octal representations.
///      - Debug trait for complex structures like arrays.
///
/// # Examples
/// ```text
/// Write smthig
/// Hello, world!
/// Hello, Alex!
/// Hello, Alex, Tom!
/// Hello Alex, Tom, Alex again!
/// Hello Alex, Tom!
/// Binary: 110010, hex: 32, octa: 62
/// Array [1, 2, 3]
/// ```
///
/// ## Panics
/// This function does not panic but gracefully handles input errors and prints a message if an error occurs.
///
/// ## Notes
/// - Ensure the `std::io` module is imported for input functionality.
/// - This example is suitable for demonstrating basic Rust I/O operations and formatted printing.
fn main() {
    //! # main function
    //! ```
    //! create var with empty string
    //! ```
    //! return user input
    let mut user_input = String::new();
    println!("Write smthig");

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            println!("You wrote: {user_input}");
        }
        Err(error) => {
            println!("Error: bad request {}", error);
        }
    }

    /// printing
    println!("Hello, world!");
    println!("Hello, {}!", "Alex");
    println!("Hello, {}, {}!", "Alex", "Tom");
    ///positinal
    println!("Hello {0}, {1}, {0} again!", "Alex", "Tom");
    ///naming
    println!(
        "Hello {firstName}, {secondName}!",
        firstName = "Alex",
        secondName = "Tom"
    );
    ///Traits
    println!("Binary: {:b}, hex: {:x}, octa: {:o}", 50, 50, 50);
    ///Debug
    println!("Array {:?}", [1, 2, 3]);
}
