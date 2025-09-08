
pub fn mul(x: i32, y: i32) -> i32 {
    return x * y;
}

/// Divides two numbers.
pub fn div(x: i32, y: i32) -> i32 {
    x / y
}


"Functions for tuple exercises"
pub fn first(t: (bool, u32, char)) -> bool {
    todo!();
}

pub fn last(t: (bool, u32, char)) -> char {
    todo!();
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    (t.1, t.0)
}


fn main() {
    let x = 6;
    let y = 3;

    println!("{} * {} = {}", x, y, mul(x, y));
    println!("{} / {} = {}", x, y, div(x, y));
}