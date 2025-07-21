/*10. Declaring String Object and converting String Literal to String Object
*/

fn main() {
    // Declaring a string literal (immutable string slice)
    let string_literal: &str = "Hello, Rust!";
    println!("String Literal: {}", string_literal);

    // Converting String Literal to String object
    let string_object: String = string_literal.to_string();
    println!("Converted String Object: {}", string_object);

    // Alternatively, using the `String::from` method to convert the string literal to a String
    let another_string_object: String = String::from(string_literal);
    println!("Another Converted String Object: {}", another_string_object);
    
    // Creating a String object directly
    let direct_string = String::from("This is a String object!");
    println!("Directly Created String: {}", direct_string);
}
