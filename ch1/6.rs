/*6. Write Program to Declare an array, arr, of size 6 that has numbers divisible by 2 ranging
from 0 to 10 and Print the value of arr. */

fn main() {
    // Declare an array of size 6 with numbers divisible by 2, ranging from 0 to 10
    let arr = [0, 2, 4, 6, 8, 10];

    // Print the array values
    println!("Array values:");
    for num in arr.iter() {
        println!("{}", num);
    }
}
