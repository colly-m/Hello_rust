#![allow(unused)]

fn add_with_return(a: i32, b: i32) -> i32 {
    return a + b;
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print(s: String) {
    println!("{s}{s}{s}{s}{s}");
} 

fn main() {
    let x = 5;
    let y = 10;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    print("👌".to_string());
}