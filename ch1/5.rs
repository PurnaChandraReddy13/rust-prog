/*5. Write a program to implement the following
a. Implicit type declaration
b. Explicit type declaration*/

fn main() {
    // a. Implicit type declaration (Type inferred by the compiler)
    let a = 42; // The compiler infers that a is of type i32
    println!("Implicit type declaration: {}", a); // Output: 42

    // b. Explicit type declaration (You specify the type explicitly)
    let b: f64 = 3.14; // Explicitly specifying the type as f64
    println!("Explicit type declaration: {}", b); // Output: 3.14

    // Implicit type for string
    let c = "Hello, Rust!"; // The compiler infers the type as &str (string slice)
    println!("Implicit type for string: {}", c); // Output: Hello, Rust!

    // Explicit type for string
    let d: &str = "Explicit string"; // Explicitly specifying the type as &str
    println!("Explicit type for string: {}", d); // Output: Explicit string
}
