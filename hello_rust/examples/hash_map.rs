#![allow(unused)]

use std::collections::HashMap;

// HashMap
fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    println!("{:#?}", scores);

    // Get
    let score: Option<&u32> = scores.get("Blue");
    println!("Blue score: {:?}", score);
    let score: Option<&u32> = scores.get("Red");
    println!("Red score: {:?}", score);

    // Update
    let score: &mut u32 = scores.entry("Black".to_string()).or_insert(0);
    *score += 1;

    let score: Option<&u32> = scores.get("Black");
    println!("Black score: {:?}", score);
}