#![allow(unused)]

fn main() {
    // variables are immutable by default
    // Use mut keyword to make a variable mutable
    let mut x = 5;
    x += 5;

    // type inference
    let y: i32 = 10;
    let z = 10;

    // Shadowing
    let x: i32 = 15;
    let x: i32 = 5;
    let x: bool = true;

    // type placeholder
    let x: _ = 5220;

    // constants
    const MAX_POINTS: u32 = 100;

    // println! macro
    let x: i32 = 10;
    println!("The value of x is: {}", x);
    // inline
    println!("The value of x is: {x}");
    // positional arguments
    println!("{0} + {0} = {1}", x, x + x);
    // \\debudg trait
    println!("The value of x is: {x:?}");
    println!("Debug: x {:?}", x);
    println!("Debug: x {:#?}", x);
}