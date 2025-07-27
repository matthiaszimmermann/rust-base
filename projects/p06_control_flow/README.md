# Control Flow

From [The Rust Programming Language, Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

## Create the Project

```bash
cd ./projects
cargo new p06_control_flow
cd p06_control_flow 
```

## Source Code

### If Statement

In Rust, unlike some other languages like C, C++, Java, or JavaScript, 
the if condition does not require parentheses around the condition expression. 
Parentheses around conditions are optional in Rust and generally omitted to improve readability.


```rust
if number < 5 {
    println!("number < 5 was true");
} else {
    println!("number < was false");
}
```

### No Ternary Operator '?'

Rust does not have an `?` operator.
So instead of let a = true ? 1 : 0; the statement given below can be used.

```rust
let a = if true { 1 } else { 0 };
```

### Loops returning Values

```rust
count = 0;
let result = loop {
    if count == 10 {
        // the value after "break" is 
        // the value returned by the loop { }
        break 2 * count;
    }
    count += 1;
};
```

Variable `result` has value 20;

### Loops with Labels

```rust
let mut count = 0;
// labeled loop
'outer_loop: loop {
    println!("count = {count}");
    let mut remaining = 10;
    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            // break without a label breaks the innermost loop
            break; 
        }
        if count == 2 {
            // directly breaks the outer loop
            break 'outer_loop;
        }
        remaining -= 1;
    }
    count += 1;
}
```