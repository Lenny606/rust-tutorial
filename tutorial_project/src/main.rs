use std::io;

fn main() {
    //! # main function
    //! ```
    //! create var with empty string
    //! ```
    //! return user input
    let mut user_input = String::new();
    println!("Write smthig");

    match io::stdin().read_line( &mut user_input ) {
        Ok(_) => {
            println!("You wrote: {user_input}");
        },
        Err(error) => {
            println!("Error: bad request {}", error);
        }

    }


    println!("Hello, world!");
}
