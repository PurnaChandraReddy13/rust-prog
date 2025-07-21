/*3. Write a program to do the following
a. Declare a variable x and store value 1000 in it.
b. Declare a variable y and store value “Programming” in it
c. Print the values of x and y
d. Change the value of x to 1100
e. Print the values of x and y*/

fn main() {
    // a. Declare a variable x and store value 1000 in it.
    let mut x = 1000;

    // b. Declare a variable y and store value "Programming" in it.
    let y = "Programming";

    // c. Print the values of x and y
    println!("x: {}, y: {}", x, y);

    // d. Change the value of x to 1100
    x = 1100;

    // e. Print the values of x and y again
    println!("x: {}, y: {}", x, y);
}
