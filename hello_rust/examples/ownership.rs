#![allow(unused)]

// Ownership rules

fn main() {
    // Owner of s is s
    let s = String::from("hello"); // s comes into scope
    // Owner of r is r
    let r = 1;

    // 2 There can only be one owner at a time
    let s = String::from("dog");
    // Owner of s is s1
    let s1 = s;
    // Owner of s is s2
    let s2 = s1;
    println!("{}", s2);

    // 3 When owner goes out of scope value is dropped
    let s = String::from("cat");
    {
        let s1 = s;
        println!("{}", s1);
    }

    // Ownership does not move for types that implement Copy trait
    let r = 1;
    // Owner of r is r1
    let r1 = r;
    // Owner of r2 is r2
    let r2 = r1;
    println!("{} {}", r1, r2);
}