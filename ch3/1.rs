/*1. Write a program to check if a number is positive or negative
*/

use std::io;

fn main() {
    // Asking the user for input
    println!("Enter a number:");

    // Getting input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Converting input to a number
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Checking if the number is positive or negative
    if number > 0 {
        println!("The number is positive.");
    } else if number < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
}
