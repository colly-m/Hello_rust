#![allow(unused)]

fn main() {
    let x: Option<i32> = Some(9);
    match x {
        Some(val) => println!("x is {}", val),
        None => {}
    }

    if let Some(val) = x {
        println!("x is {}", val);
    }
}