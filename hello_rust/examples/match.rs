#![allow(unused)]

fn main() {
    // match
    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    
    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }

    let x = 10;
    match x {
        i @ 1..=10 => println!("1 to 10 {}", i),
        _ => println!("other"),
    }

    let x: Option<i32> = Some(9);

}