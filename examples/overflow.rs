#![allow(unused)]

// overflows doesn't panic in release mode
fn main() {
    let mut x = u32::MAX;
    x += 1;

    println!("u32 max: {}, x: {}", u32::MAX, x);

    // u32::checked_add - return None on overflow
    // u32::wrapping_add - explicitly allow overflow
}