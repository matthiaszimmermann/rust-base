fn main() {
    println!("control flow demos");
    if_expressions();
    loops();
    while_loops();
    for_loops();
}


fn if_expressions() {
    println!("\nif expressions");
    let number = 3;
    if number == 0 {
        println!("- {number} == 0 was true");
    } else if number < 5 {
        println!("- {number} < 5 was true");
    } else {
        println!("- {number} < was false");
    }

    let none_u8 = None::<u8>;
    if none_u8.is_none() {
        println!("- None::<u8>.is_none() is true");
    } else {
        println!("- None::<u8>.is_none() is false");
    }

    // conditional assignment, no ternary operator ? in rust
    // last expression (no trailing semicolon) in if arm is taken as return value of the if statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // escaping for {  is {{
    println!("- if true {{ 5 }} else {{ 6 }} = {number}");
}


fn loops() {
    println!("\nloops");
    let mut count = 0;
    loop {
        println!("count: {count}");
        if count >= 2 {
            println!("breaking loop");
            // omitting the semicolon only works when break/continue is the last expression in the block
            break
        }
        count += 1;
    }

    // loop returning a value
    count = 0;
    let result = loop {
        if count == 10 {
            // the expression after "break" defines the value returned by the block { }
            break 2 * count
        }
        count += 1;
    };
    println!("loop value: {result}");

    // nested loops
    let mut count = 0;
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
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");    
}


fn while_loops() {
    println!("\nwhile loops");
    let mut count = 3;
    while count != 0 {
        println!("{count}...");
        count -= 1;
    }
    println!("LIFTOFF!!!");
}


fn for_loops() {
    println!("\nfor loops");
    // looping over array elements
    let a = [10, 20, 30];
    let mut count = 0;
    for element in a {
        println!("element/{count}: {element}");
        count += 1;
    }
    // looping over range
    for i in 0 .. a.len() {
        println!("a[{i}]: {}", a[i]);
    }
}
