fn main() {
    println!("integer literals");
    println!("98_222: {}", 98_222);
    println!("0xff: {}", 0xff);
    println!("0b0000_0110: {}", 0b0000_0110);
    println!("b'x': {}", b'x');

    println!("\nsigned type: i8");
    let i8 :i8 = 0;
    println!("i8 + 1 = {}, i8 - 1 = {}", i8 + 1, i8 - 1);
    println!("i8.wrapping_sub(1) = {}", i8.wrapping_sub(1));
    println!("i8.checked_sub(1) = {:?}", i8.checked_sub(1));
    println!("i8.overflowing_sub(1) = ({},{})", i8.overflowing_sub(1).0, i8.overflowing_sub(1).1);
    println!("i8.saturating_sub(1) = {}", i8.saturating_sub(1));

    println!("\nunsigned type: u8");
    let mut u8: u8 = 0;
    println!("u8 + 1 = {}", u8 + 1);
    println!("u8.wrapping_sub(1) = {}", u8.wrapping_sub(1));
    println!("u8.checked_sub(1) = {:?}", u8.checked_sub(1));
    println!("u8.overflowing_sub(1) = ({},{})", u8.overflowing_sub(1).0, u8.overflowing_sub(1).1);
    println!("u8.saturating_sub(1) = {}", u8.saturating_sub(1));

    println!("\nfloating point types");
    let one_third64 = 1.0 / 3.0; // f64, both operands need to be floats
    let one_third32: f32 = 1.0 / 3.0; // f32
    println!("oneThird64 {one_third64}");
    println!("oneThird32 {one_third32}");

    println!("\ncasting:");
    u8 = 128;
    let u16 = 111;
    println!("u16::from(u8) = {}", u16::from(u8)); // widening cast -> from()
    println!("u8::try_from(u16).unwrap() = {}", u8::try_from(u16).unwrap()); // narrowing cast -> try_from() + unwrap

    println!("\nnumeric operations");
    println!("sum: 5 + 10 = {}", 5 + 10);
    println!("difference: 95.5 - 4.3 = {}", 95.5 - 4.3);
    println!("product: 4 * 30 = {}", 4 * 30);
    println!("quotient: 56.7 / 32.2 = {}", 56.7 / 32.2);
    println!("truncated: -5 / 3 = {}", -5 / 3);
    println!("remainder: 43 % 5 = {}", 43 % 5);

    println!("\nbooleans");
    println!("true: {}", true);
    println!("false: {}", false);
    println!("!true: {}", !true);
    println!("true && false: {}", true && false);
    println!("true || false: {}", true || false);

    println!("\ncharacters (unicode)");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}");
    println!("z: {z}");
    println!("heart_eyed_cat: {heart_eyed_cat}");

    println!("\ntuples");
    let tup: (u8, char, f64) = (255, 'x', -0.43);
    let (u, c, f) = tup;
    println!("tup (u,c,f): ({u},{c},{f})");
    println!("u: {u}, tup.0: {}", tup.0);
    println!("c: {c}, tup.1: {}", tup.1);
    println!("f: {f}, tup.2: {}", tup.2);

    println!("\narrays");
    let arr = [1, 2, 3, 4, 5]; // arrays have a fixed length
    println!("arr.len(): {}", arr.len());
    println!("arr[0]: {}", arr[0]);
    println!("arr[1]: {}", arr[1]);
    // println!("arr[5]: {}", arr[5]); // compiler out of bounds error
}
