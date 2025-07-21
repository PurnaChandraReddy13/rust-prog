/*2. Write a program to implement Borrowing and Dereferencing Operators
*/

fn main() {
    // Immutable borrowing
    let x = 10;          // Declare a variable `x`
    let y = &x;          // Borrow a reference to `x` (immutable borrow)

    println!("Value of x through immutable borrow (y): {}", y); // Dereferencing `y` by printing it
    println!("Value of x directly: {}", x);

    // Mutable borrowing
    let mut z = 20;      // Declare
