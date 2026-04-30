#![allow(unused)]

// Memory - stack and heap
// Stack
// - Stores data of fixed size known at compile time
// - Fast access
// - Last In First Out (LIFO) structure
// - Limited size
// Heap
// - Stores data of variable size or unknown size at compile time
// - Slower access due to pointer indirection
// - More flexible, can grow and shrink as needed
// - Requires manual memory management or garbage collection

fn main() {
    // Stack
    let x = 5;
    let arr: [i32; 10] = [1; 10];

    // Heap
    let mut s: String = "hello".to_string();
    s += " world";

    let mut v: Vec<i32> = vec![];
    v.push(1);
    v.push(2);
    v.push(3);

    let boxed = Box::new(42);
}