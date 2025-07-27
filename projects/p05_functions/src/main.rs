use std::any::{Any, TypeId, type_name};

fn main() {
    println!("Hello, world!");
    some_function(1, 42);
}

fn some_function(x: u32, y:u16) {
    println!("Hi from another function. x: {x}, y: {y}");
    println!("Type of x: {}", inspect_type(x));
    println!("Type of y: {}", inspect_type(y));

    // widening cast: into(), eg, u16 -> u32, does not need unwrap
    // narrowing cast: try_into
    let x16 :u16 =  x.try_into().unwrap();
    println!("x + y = {}", a_plus_b(x16, y));
}

fn a_plus_b(a: u16, b:u16) -> u16 {
    return a + b;
}

fn inspect_type<T: 'static>(_: T) -> String {
    if TypeId::of::<T>() == TypeId::of::<u32>() {
        return "It's a u32!".to_string();
    } else if TypeId::of::<T>() == TypeId::of::<String>() {
        return "It's a String!".to_string();
    } else {
        return ("Some other type: ".to_owned() + type_name::<T>()).to_string();
    }
}