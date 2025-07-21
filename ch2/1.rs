/*1. Write a program to implement Type Casting Operator.
*/

fn main() {
    // Integer to Float Casting
    let integer_value: i32 = 10;
    let float_value: f32 = integer_value as f32; // Casting i32 to f32
    println!("Integer to Float: {} -> {}", integer_value, float_value);

    // Float to Integer Casting
    let float_value2: f64 = 3.14;
    let integer_value2: i32 = float_value2 as i32; // Casting f64 to i32 (will truncate the decimal)
    println!("Float to Integer: {} -> {}", float_value2, integer_value2);

    // Char to Integer Casting (ASCII value)
    let character: char = 'A';
    let ascii_value: u8 = character as u8; // Casting char to u8 (ASCII value)
    println!("Char to Integer (ASCII): {} -> {}", character, ascii_value);

    // Integer to Char Casting (based on ASCII value)
    let number: i32 = 66;
    let character_from_int: char = number as u8 as char; // Casting i32 to u8, then to char
    println!("Integer to Char: {} -> {}", number, character_from_int);

    // From Larger Integer Type to Smaller (i64 to i32)
    let large_integer: i64 = 100_000_000_000;
    let smaller_integer: i32 = large_integer as i32; // Casting i64 to i32 (will cause overflow if value is too large)
    println!("Large Integer to Smaller Integer: {} -> {}", large_integer, smaller_integer);
}
