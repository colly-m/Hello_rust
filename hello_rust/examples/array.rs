#![allow(unused)]

// Array = collection of elements of the same type
// Slice = collection of elemnts with length unknown at compile time
fn main() {
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]: {}", arr[0]);
    // Write
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[0] = 10;

    let arr: [i32; 10] = [0; 10]; // all elements initialized to 0
    println!("arr: {:?}", arr);

    // Slice - reference to a contiguous sequence of elements in an array
    // Slice 
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    // First 3
    let s: &[i32] = &nums[0..3];

    // Last 3
    let s: &[i32] = &nums[7..10];

    // Middle 4.
    let s: &[i32] = &nums[3..7];
    println!("mid 4: {:?}", s);
}