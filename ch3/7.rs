/*7. Write a program to Count Iterations of a Loop Until a Condition
*/

fn main() {
    let mut count = 0;

    // Loop that continues until the count reaches 5
    loop {
        count += 1;
        println!("Iteration: {}", count);

        // Condition to break the loop
        if count == 5 {
            break;
        }
    }

    println!("Loop finished after {} iterations.", count);
}
