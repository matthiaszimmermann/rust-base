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

### Ordering of Functions

Ordering of functions in a module: `main()` is placed at the top. main() is followed by functions it calls, ordered by the call hierarchy so readers can follow the logic top-down. 

### Functions with Return Values

The last expression in a Rust function is automatically used as the return value.

This works ...

```rust
fn a_plus_b(a: u16, b: u16) -> u16 {
    return a + b;
}
```

However, preferred is the following approach:

```rust
fn a_plus_b(a: u16, b: u16) -> u16 {
    a + b
}
```

Be sure to **leave off the semicolon on the last expression** so it’s treated as the return value instead of a statement.