/*4. Write a program to Match a pattern using If Let Expression.*/

fn main() {
    let some_value = Some(10);

    if let Some(x) = some_value {
        println!("Matched a value: {}", x);
    } else {
        println!("No match found.");
    }

    let another_value = None;

    if let Some(y) = another_value {
        println!("Matched a value: {}", y);
    } else {
        println!("No match found.");
    }
}
