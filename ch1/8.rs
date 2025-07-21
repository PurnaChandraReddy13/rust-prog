/*. Write a program to create an array of 10 elements and implement the following
a. Create a of 2nd and 3rd element
b. Omit the start index of the slice
c. Omit the End Index of the Slice
d. Omit both Start and End Index of the Slice*/

fn main() {
    // Create an array of 10 elements
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // a. Create a slice of the 2nd and 3rd elements (indexes 1 and 2)
    let slice1 = &arr[1..3]; // Slice from index 1 to index 3 (excluding index 3)
    println!("Slice of 2nd and 3rd elements: {:?}", slice1);

    // b. Omit the start index of the slice (starts from index 4 to the end)
    let slice2 = &arr[4..]; // Slice from index 4 to the end of the array
    println!("Slice starting from index 4: {:?}", slice2);

    // c. Omit the end index of the slice (starts from index 3 to the end)
    let slice3 = &arr[..6]; // Slice from the start to index 6 (excluding index 6)
    println!("Slice up to index 6: {:?}", slice3);

    // d. Omit both start and end index of the slice (entire array)
    let slice4 = &arr[..]; // Slice from the start to the end (entire array)
    println!("Entire array slice: {:?}", slice4);
}
