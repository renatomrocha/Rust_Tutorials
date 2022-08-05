/*
Primitive Types
Integers: u8, i8, i16, u16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: (bool)
Characters (char)
Tuples
Arrays -> Fixed size
*/

// Rust is a statically typed language, which meand that it must know the types of all variables at compile time,
// however, the compiler can usually infer what type we want to use based on the value and how we use it


pub fn run() {

    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45455454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    // Boolean from expression
    let is_greater = 10 > 5;


    // char (unicode character)
    let char_1= 'a'; // -> single quotes for char

    let face : char = '\u{1F600}';


    println!("{:?}", (x, y, z, is_active, is_greater, char_1, face));

}