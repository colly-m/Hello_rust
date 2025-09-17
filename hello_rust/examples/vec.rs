#![allow(unused)]

// Vector
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);

    let v = vec![1, 2, 3];
    println!("Vector: {:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Vector: {:?}", v);

    let v: Vec<i8> = vec![0i8; 100];
    println!("v: {:?}", v);

    // Pop - remove last element
    let mut v = vec![1, 2, 3];
    let x: Option<i32> = v.pop();
    println!("pop: {:?}", x);


    // Slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..4];
    println!("slice: {:?}", s);

}