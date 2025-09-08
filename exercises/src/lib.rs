
pub fn mul(x: i32, y: i32) -> i32 {
    return x * y;
}

/// Divides two numbers.
pub fn div(x: i32, y: i32) -> i32 {
    x / y
}

fn main() {
    let x = 6;
    let y = 3;

    println!("{} * {} = {}", x, y, mul(x, y));
    println!("{} / {} = {}", x, y, div(x, y));
}