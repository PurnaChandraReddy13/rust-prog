/*6. Write a program to Multiplication Table using Loop Labels
*/

fn main() {
    let n = 5; // The number for which we want the multiplication table

    'outer: for i in 1..=10 {
        println!("{} times:", n);

        for j in 1..=10 {
            println!("{} x {} = {}", n, j, n * j);
            if j == 10 {
                break 'outer; // Break out of the outer loop after printing the table for the given number
            }
        }
    }
}
