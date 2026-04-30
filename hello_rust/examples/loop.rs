#![allow(unused)]

fn main() {
    let mut i = 0;
    loop{
        println!("again!");
        i += 1;
        if i > 5 {
            break;
        }
    }

    let mut i = 0;
    while i <= 5 {
        println!("yet again!");
        i += 1;
    } 

    for i in 0..6 {
        println!("for again! {}", i);
    }

    let arr = [10, 20, 30, 40, 50];

    let n: usize = arr.len();
    for i in 0..n {
        println!("arr[{}] = {}", i, arr[i]);
    }

    for n in arr {
        println!("n = {}", n);
    }

    let v = vec![100, 200, 300, 400, 500];

    for n in v.iter() {
        println!("n {}", n);
    }

    for n in v.iter() {
        println!("n {}", n);
    }

    let mut i = 0;
    let z: &str = loop{
        println!("again!");
        i += 1;
        if i > 5 {
            break "look it ends";
        }
    };
    println!("{}", z);
}