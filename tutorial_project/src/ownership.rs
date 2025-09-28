//! Ownership and Strings demo: String creation, mutation, and borrowing.
#![allow(dead_code)]

pub fn print_text(boat_name: String, dog: &str) -> String {
    let text = format!(
        "I'm owner of {0}, will be sailing with my {1}",
        boat_name, dog
    );
    println!("{}", text);
    text
}

pub fn print_mutable_text(text: &mut String) {
    *text = String::from("changed text");
}

pub fn demo_ownership() {
    // String object examples
    let mut boat = String::new(); // mutable String
    boat.push('B'); // single char
    boat.push_str("New Name");

    let boat_name = String::from("Jenny");
    let dog = "DOG";

    let mut new_text = String::from("Distro");

    let _returned = print_text(boat_name, dog); // moves boat_name
    print_mutable_text(&mut new_text); // borrows mutably
}
