
pub fn mul(x: i32, y: i32) -> i32 {
    return x * y;
}

/// Divides two numbers.
pub fn div(x: i32, y: i32) -> i32 {
    x / y
}


// Functions for tuple exercises
pub fn first(t: (bool, u32, char)) -> bool {
    todo!();
}

pub fn last(t: (bool, u32, char)) -> char {
    todo!();
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    (t.1, t.0)
}

pub fn zeros() -> [u32; 100] {
    [0; 100]
}

pub fn first_3(s: &[u32]) -> &[u32] {
    &s[0..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
    &s[s.len()-3..s.len()]
}


pub fn hello() -> String {
    // Function to return "Hello Rust"
    "Hello Rust".to_string()
}


pub fn greet(name: &str) -> String {
    // function to return "Hello" appended with name
    format!("Hello, {}!", name)
}


pub fn append(mut s: String) -> String {
    // function to append "!" to s
    s.push('!');
    s
}


pub enum Color {
    // Prefix `enum`with `pub`
    // Make the variants `public
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
}

#[derive(Debug)]
pub struct Account {
    // Struct named Account with address and balance fields
    pub address: String,
    pub balance: u32,
}

pub fn new(address: String) -> Account {
    // Function to create a new account with 0 balance
    Account {
        address,
        balance: 0,
    }
}

pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
    // Function to create a vector with x, y, z
    vec![x, y, z]
}

fn main() {
    let x = 6;
    let y = 3;

    println!("{} * {} = {}", x, y, mul(x, y));
    println!("{} / {} = {}", x, y, div(x, y));
}