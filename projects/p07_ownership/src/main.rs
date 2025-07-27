fn main() {
    println!("ownership and references");
    ownership();
    references();
}

fn ownership() {
    println!("\nownership");

    // s1 has ownership over newly created String
    let s1 = String::from("hello");
    println!("- s1 owner: {s1}, world!"); 

    // moves ownership to s2 (s1 looses ownership, ie s1 goes out of scope)
    let s2 = s1;

    // using s2 this works
    println!("- s2 owner: {s2}, world!"); 

    // attempting to use s1 here  leads to a compile error
    // println!("- s1 owner: {s1}, world!");

    // cloning/creating a deep copy
    let s3 = s2.clone();
    println!("- s2 owner: {s2}, world!"); 
    println!("- s3 owner of s2 clone: {s3}, world!"); 

    // ownership in function then, moved to s1
    let s10 = gives_ownership(); 
    println!("- s10 obtained ownership from fn: '{s10}'"); 

    // ownership of s10 moved to fn and back from fn to s11
    // terminates scope of s10
    let mut s11 = takes_and_gives_back(s10);
    println!("- s11 obtained ownership from fn: '{s11}'");

    // bad approach, each function call needs re-assigning all argument vars
    let mut len: usize = 0;
    (s11, len) = calculate_length_bad(s11);
    println!("- s11 len = {len}, s11: {s11}");
}


fn gives_ownership() -> String { 
    // function is owner of newly created string
    let some_string = String::from("gift from 'fn gives_ownership'");
    // moves ownership to caller
    some_string
}


fn takes_and_gives_back(a_string: String) -> String {
    println!("- fn takes_and_gives_back now owner of: '{a_string}'"); 
    // moving ownership back to caller
    a_string
}


fn calculate_length_bad(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}


fn references() {
    immutable_references();
    mutable_references();
}


fn immutable_references() {
    println!("\nimmutable references");
    let s = String::from("hello");

    // passing by reference, default (read-only) case
    // adding '&' defines by reference argument
    let len = calculate_length(&s);
    println!("- the length of '{s}': {len}");

    // creating (immutable) references to variables
    let r1 = &s; // no problem
    let r2 = &s; // also not a problem

    // no problem to create as many references as needed
    // as long as they remain immutable
    let r3 = &s; 
    println!("r1: {r1}, r2: {r2} and r3: {r3}");
}


// by reference argument &String: ownership is not transferred
// the function 'borrows' variable without having ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}


fn mutable_references() {
    println!("\nmutable references");

    // passing by reference, mutable case
    let mut s = String::from("hello world");
    println!("- s before calling add_text:'{s}'");
    
    add_text(&mut s);
    println!("- s after calling add_text:'{s}'");

    let r1 = &mut s; // r1 holds mutable reference
    let r2 = &mut s; // r2 takes over mutable reference, r1 out of scope

    // does not compile as max one mutable reference is allowed at any time
    // println!("{}, {}", r1, r2);
    // same here, does not compile for the same reason
    // println!("{}, {}", s, r2); 

    // this works
    println!("- mutable reference r2: {r2}");

    // adding immutable references works too, of course
    let ri1 = &r2; // no problem
    let ri2 = &r2; // also not a problem
    println!("- mutable reference r2: {r2}, immutable references ri1: {ri1}, ri2: {ri2}");
}


// &mut authorizes function to change value of argument
fn add_text(s: &mut String) {
    s.push_str(", world");
}
