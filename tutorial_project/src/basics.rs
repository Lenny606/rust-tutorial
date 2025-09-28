//! Basics demo: variables, shadowing, tuples, chars, and simple prints.
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn demo_basics() {
    // Variables and shadowing
    let name = "Tom"; // immutable
    let name = "Alex"; // shadowed
    let mut name_second = "Dave"; // mutable
    let (a, b, c) = (12, "string", 5); // tuple destructuring

    // Characters
    let _char_b = 'B';
    let smile = '\u{1F600}'; // unicode
    println!("Smile Emoji: {}", smile);

    // String slices (immutable views into string data)
    let _cat = "CAT";
    let _dog: &'static str = "DOG"; // 'static lifetime example
}
