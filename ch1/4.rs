/*4. Write a program to implement the Scope and Shadowing*/

fn main() {
    // Declare a variable in the main scope
    let x = 10;
    println!("Value of x in main scope: {}", x); // Output: 10
    
    {
        // This is a new scope
        let x = 20; // Shadowing the x variable
        println!("Value of x inside inner scope: {}", x); // Output: 20
    }

    // The outer x variable is still valid here
    println!("Value of x in outer scope: {}", x); // Output: 10
    
    // Shadowing the variable again
    let x = "Now I'm a string!";
    println!("Shadowed x value: {}", x); // Output: Now I'm a string!
}

