#![allow(unused)]

fn main() {
    // Panic! ("Something went wrong")

    let v = vec![1, 2, 3];
    // indexing out of bounds will cause a panic

    let x = v.get(1);
    match x {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }

    // Result<T, E> = Ok(T) | Err(E)
    let x = 1;
    let y = 0;
    // This will cause a panic
    let q = x / y;
    println!("{:?}", q);

    let q: Result<i32, String> = if y == 0 {
        Err(String::from("Divide by zero"))
    } else {
        Ok(x / y)
    };
}