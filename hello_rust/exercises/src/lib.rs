use std::collections::HashMap;

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

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    // Function to initialize a hashmap, insert amount value at the key address
    let mut map = HashMap::new();
    map.insert(address, amount);
    map
}


// If_else exercises

pub fn min(x: i32, y: i32) -> i32 {
    // Function to return the minimum of both x and y
    if x < y {
        x
    } else {
        y
    }
}

pub fn max(x: i32, y: i32) -> i32 {
    // Function to return the maximum of both x and y
    if x > y {
        x
    } else {
        y
    }
}

pub fn sign(x: i32) -> i32 {
    // Function to return sign of x
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}


// Loop exercises

pub fn sum(nums: Vec<i32>) -> i32 {
    // Returns sum of all integres in nums vector
    let mut total = 0;
    for num in nums {
        total += num;
    }
    total
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    // Returns a vector of length n, filled with the value i
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(i);
    }
    v
}


// Match exercise
pub fn num_to_string(num: u32) -> String {
    // Function to convert num into String, "one" to "three"
    match num {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        _ => "other".to_string(),
    }
}

pub fn unwrap_or_default(x: Option<u32>, v:u32) -> u32 {
    // Function to extract value wrapped in Some, if x is None returning v
    match x {
        Some(val) => val,
        None => v,
    }
}


// if_let exercise
pub fn unwrap_or_default(x: Option<u32>, v:u32) -> u32 {
    // Function to extract va`lue wrapped in Some, if x is None returning v
    if let Some(val) = x {
        val
    } else {
        v
    }
}


// Ownership rules
pub fn exercise_1() {
    let s = "rust".to_string();
    let s1 = s;
    // let s2 = s;
    println!("{}", s1);
}

pub fn exercise_2() {
    let s = "rust".to_string();
    {
        let s1 = s;
        println!("{}", s1);
    }
    // println!("{}", s);
}

pub fn exercise_3() {
    let s = "rust".to_string();
    take(s);
    // println!("{}", s);
    println!("{s}");
}


pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    // Return MathError if y is 0. Otherwise return x/y
    if y == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}


pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    // Return v[i] if i is a valid index. Or return default_val
    if i < v.len() {
        v[i]
    } else {
        default_val
}


pub fn parse_and_add(a: &str, b: &str) -> u32 {
    // Parse a and b into u32 and return the sum
    let a: u32 = a.parse().unwrap();
    let b: u32 = b.parse().unwrap();
    a + b
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    // Call unwrap to get innner values of x and y returning their sum
    let x = x.unwrap();
    let y = y.unwrap();
    x + y
}


pub fn sum(nums: &[&str]) -> Result<u32, String> {
    // Parse the slice of string into u32 and return their sum
    let mut total = 0;
    for num in nums {
        let num: u32 = match num.parse() {
            Ok(val) => val,
            Err(_) => return Err("Invalid number".to_string()),
        };
        total += num;
    }
    Ok(total)
}


fn main() {
    let x = 6;
    let y = 3;

    println!("{} * {} = {}", x, y, mul(x, y));
    println!("{} / {} = {}", x, y, div(x, y));
}