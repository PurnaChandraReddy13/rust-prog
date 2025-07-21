/*2. Write a program to implement Borrowing and Dereferencing Operators
*/
fn main() {
    // Immutable borrowing
    let x = 10;          // Declare a variable `x`
    let y = &x;          // Borrow a reference to `x` (immutable borrow)

    println!("Value of x through immutable borrow (y): {}", y); // Dereferencing `y` by printing it
    println!("Value of x directly: {}", x);

    // Mutable borrowing
    let mut z = 20;      // Declare a mutable variable `z`
    let w = &mut z;      // Mutable borrow of `z`

    *w += 5;             // Dereference `w` to modify the value of `z`
    println!("Value of z after mutable borrow and modification: {}", z); // Prints 25

    // Demonstrating ownership and borrowing in a function
    let a = String::from("Hello, Rust!"); // String is heap-allocated

    // Immutable borrow
    let b = borrow_string(&a); 
    println!("Borrowed string: {}", b); // Dereferencing borrowed string

    // Mutable borrow
    let mut c = String::from("Hello");
    modify_string(&mut c);
    println!("Modified string: {}", c); // Prints modified string
}

// Function demonstrating immutable borrowing
fn borrow_string(s: &String) -> &str {
    s.as_str() // Borrowing the string and returning a string slice
}

// Function demonstrating mutable borrowing
fn modify_string(s: &mut String) {
    s.push_str(", World!"); // Modifying the borrowed string
}

