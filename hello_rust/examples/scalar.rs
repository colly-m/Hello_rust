#![allow(unused)]

// Scalar = data types that represent a single value
// Scalar types in Rust: integers, floating-point numbers, Booleans, and characters
fn main() {
    // Signed integers
    // Range: -2^(n-1) to 2^(n-1)-1
    // -2^(8-1) ~ 2^(8-1) - 1
    let i0: i8 = -1;
    //let -2^(16-1) ~ 2^(16-1) - 1
    let i1: i16 = 2;
    //let -2^(32-1) ~ 2^(32-1) - 1
    let i2: i32 = 3;
    //let -2^(64-1) ~ 2^(64-1) - 1
    let i3: i64 = 4;
    //let -2^(128-1) ~ 2^(128-1) - 1
    let i4: i128 = 5;

    // Unsigned integers
    // Range: 0 to 2^n-1
    // 0 ~ 2^(8) - 1
    let u0: u8 = 1;
    // 0 ~ 2^(16) - 1
    let u1: u16 = 2;
    // 0 ~ 2^(32) - 1
    let u2: u32 = 3;
    // 0 ~ 2^(64) - 1
    let u3: u64 = 4;
    // 0 ~ 2^(128) - 1
    let u4: u128 = 5;

    // Depending on computer architecture
    let i5: isize = -6;
    let u5: usize = 7;

    // Floating-point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.021;

    // Boolean
    let t: bool = true;
    let f: bool = false;

    // Character
    let c0: char = 'a';
    let c1: char = '中';
    let c2: char = '😊';

    // type conversion
    let i: i32 = -5;
    let u: u32 = i as u32; // -5 as u32 will be a large number due to underflow
    println!("i: {}, u: {}", i, u);

    // min to max
    let i_max = i32::MAX;
    let i_min = u32::MIN;
    println!("i max: {i_max}");
    println!("u min: {i_min}");
}