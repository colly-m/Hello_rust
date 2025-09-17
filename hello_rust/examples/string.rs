#![allow(unused)]

// String &str (string slice)
// - Use `String` when you need ownership and mutability.
// - Use `&str` when you need a reference to a string, often for function parameters.
// - `String` is heap-allocated, growable, and UTF-8 encoded.
fn main() {
    let msg: String = String::from("Hello, Rust!");
    let msg: String = "Hello, Rust!".to_string();

    let length: usize = msg.len();

    let msg: String = String::from("Hello Rust");
    let s: &str = &msg[0..5];
    println!("s: {}", s);

    let s = "Hello World";
    let x: String = s.to_string();

    // Rust automatically converts &String into a &str when needed
    let msg: String = String::from("Hello, Rust!");
    print(&msg);

    let s: &str = "Hello, wORLD!";
    print(s);

    // Append &str to String
    let mut msg: String = String::from("Hello Rust");
    msg += " World";
    println!("{msg}");

    // String interpretation - format!
    let lang = "Rust";
    let emoji = "🦀";
    let s = "Hello Rust 🦀";
    let mut s = "Hello".to_string();
    s += " ";
    s += lang;
    s += " ";
    s += emoji;
    let s = format!("Hello {} {}", lang, emoji);
    println!("{s}");

}

fn print(s: &str) {
    println!("{s}");
}