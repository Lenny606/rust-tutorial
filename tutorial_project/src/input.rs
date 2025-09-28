//! Input demo: reads a line from stdin and demonstrates match-based error handling.
#![allow(dead_code)]
use std::io::{self, Read};

/// Reads a line from standard input and returns it.
/// This function is separated so it can be unit-tested by injecting input via stdin redirection.
pub fn read_line_from_stdin() -> io::Result<String> {
    // Using read_to_end would block until EOF; here we use read_line-like behavior via read_until('\n').
    // But the simplest is to use std::io::stdin().read_line.
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

pub fn demo_input() {
    println!("Write something:");
    match read_line_from_stdin() {
        Ok(text) => {
            let text = text.trim_end();
            if !text.is_empty() {
                println!("You wrote: {}", text);
            } else {
                println!("No input provided (empty line)");
            }
        }
        Err(err) => {
            println!("Error reading input: {}", err);
        }
    }
}
