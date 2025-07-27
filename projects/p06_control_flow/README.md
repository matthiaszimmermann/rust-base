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

### Generalization: Blocks as Expressions

Rust’s blocks more than just scopes or statement groups — they are expressions producing values.

Blocks consists of zero or more statements followed optionally by a final expression (called the "tail expression") without a semicolon.
The tail expression’s value becomes the value of the whole block.

- If the last element in the block is an expression without a trailing semicolon, that expression’s value and type is the value and type of the block expression.

- If the block ends with a statement (which usually ends with a semicolon) or has no expression, then the block’s value is () (the unit type).

- This applies recursively: since blocks themselves are expressions, a block can end with another block expression, whose value then defines the outer block value.

```rust
let x = {
    let a = 10;    // statement (ends with ;)
    let b = 20;    // another statement
    a + b          // final expression, the block's value
};
println!("{}", x); // prints 30
```

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