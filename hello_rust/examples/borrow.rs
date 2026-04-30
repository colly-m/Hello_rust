#![allow(unused)]

fn takes(s: String) {
    println!("take {s}");
}

fn borrow(s: &String) {
    println!("borrow {s}");
}

// Borrow - temporary use a value without taking ownership
// Borrowing is done using references (&)

fn main() {
    // Take ownership
    let s = String::from("rust");
    borrow(&s);
    takes(s);
    // s is no longer valid here and does not compile
    // println!("{s}");

    // - Create a reference (either mutable or immutable)
    let mut s = String::from("rust");
    let s1 = &mut s; // immutable reference
    // s1 has read and write access to s
    // let s2 = &s; // immutable reference
    // let s3 = s2;
    s1.push_str("🦀");
    let s2 = &mut s; // mutable reference
    s2.push_str("🦀");

    let mut s = String::from("rust");
    // s1, s2 and s3 have read-only access to s
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s;
    println!("s1: {s1}");
    // s3.push_str("🦀");

    // - Reference must not outlive the value
    let s = String::from("rust");
    let s1 = &s;
    {
        let s2 = &s;
        println!("s2: {s2}");
    }


    // Doesnt take ownership
    // Immutable reference - any number of read-only access to a value
    // Mutable reference - only one write access to a value at a time
    // Either immutable or mutable references, not both at the same time
    // Reference must not outlive the value
}

//fn dangle(s: String) -> &String {
//    &s
//}