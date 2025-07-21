/*7. Write a program to create and access a tuple.
*/

fn main() {
    // Creating a tuple
    let my_tuple = (42, "Hello, Rust!", 3.14, true);

    // Accessing tuple elements by indexing (using 0-based index)
    println!("First element: {}", my_tuple.0);  // Accessing the first element (42)
    println!("Second element: {}", my_tuple.1); // Accessing the second element ("Hello, Rust!")
    println!("Third element: {}", my_tuple.2);  // Accessing the third element (3.14)
    println!("Fourth element: {}", my_tuple.3); // Accessing the fourth element (true)

    // Destructuring the tuple into individual variables
    let (a, b, c, d) = my_tuple;
    println!("\nDestructured values:");
    println!("a: {}", a); // 42
    println!("b: {}", b); // "Hello, Rust!"
    println!("c: {}", c); // 3.14
    println!("d: {}", d); // true
}
