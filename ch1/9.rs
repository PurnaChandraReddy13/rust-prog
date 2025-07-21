/*9. Write a program to create different types of constants print it in the output*/

fn main() {
    // Integer constant
    const MY_INT: i32 = 42;
    println!("Integer constant: {}", MY_INT);

    // Floating-point constant
    const MY_FLOAT: f64 = 3.14159;
    println!("Floating-point constant: {}", MY_FLOAT);

    // Boolean constant
    const MY_BOOL: bool = true;
    println!("Boolean constant: {}", MY_BOOL);

    // Character constant
    const MY_CHAR: char = 'R';
    println!("Character constant: {}", MY_CHAR);

    // String constant
    const MY_STRING: &str = "Rust Programming";
    println!("String constant: {}", MY_STRING);

    // A constant representing the maximum size of a type
    const MAX_SIZE: usize = std::mem::size_of::<i32>();
    println!("Size of an i32 in bytes: {}", MAX_SIZE);
}
