/*2.Write a program to display Output following pattern using Placeholders
1
22
333
4444
55555*/

fn main() {
    for i in 1..=5 {
        // Print the number i, i times on the same line
        println!("{}", i.to_string().repeat(i as usize));
    }
}
