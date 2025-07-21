/*3. Write a program to make a calculator using Match Expression
*/

use std::io;

fn main() {
    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number!");

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number!");

    println!("Choose an operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    let result = match operation.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Division by zero!");
                return;
            }
        }
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    println!("Result: {}", result);
}
