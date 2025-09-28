compile file then run file
- rustc <filename>

new Cargo project
- cargo new <name>
    
- cargo build
- cargo run
- cargo clean
- cargo check
- cargo doc

---

Tutorial project structure (src/):
- main.rs: orchestrates the demos.
- formatting.rs: println! formatting examples (positional, named, binary/hex/octal, debug).
- input.rs: simple stdin read example with match-based error handling.
- basics.rs: variables, shadowing, tuples, chars, and slices.
- ownership.rs: String ownership/borrowing examples; mutation via &mut.
- operators.rs: constants and bitwise operator examples.

How to run:
- cargo run
- To try interactive input, open src/main.rs and uncomment `input::demo_input();`.