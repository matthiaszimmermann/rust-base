# Functions

From [The Rust Programming Language, Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## Create the Project

```bash
cd ./projects
cargo new p05_functions
cd p05_functions 
```

## Source Code

The `fn main()` function is special and only appears once in a Rust binary crate and serves as the entry point for the executable program.
It is expected to be found in the `src/main.rs` file. 

Ordering of functions in a module: `main()` is placed at the top. main() is followed by functions it calls, ordered by the call hierarchy so readers can follow the logic top-down. 