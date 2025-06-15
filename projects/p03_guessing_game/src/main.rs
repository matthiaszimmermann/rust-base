use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 'let' creates a variable (immutable per default!)
    // random_range(from..=to) creates a range including the specified lower and upper bounds
    let secret_number = rand::rng()
        .random_range(1..=10);

    loop {
        println!("try to guess a number, so input your guess.");

        // 'let mut' creates a variable that can change its value
        // String::new calls the new function implemented on the String type
        let mut guess = String::new();

        io::stdin()
            // &: passed as reference
            // &mut the parameter is passed by reference and can be modified by the function
            // 'read_line(....) returns a Result (to encode error info)
            // Result is an enum that can hold an Err or an Ok variant.
            // the function expect(...) makes the program crash if the function 
            // before returns an Err variant.
            .read_line(&mut guess)
            .expect("failed to read you guess");

        let input = guess.trim();
        // shadow/reuse variable name
        // match compares the provided value against the "arms" in the { ... }
        let guess: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("'{input}' is not a number. try again.");
                continue;
            },  
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
