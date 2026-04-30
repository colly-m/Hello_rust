#![allow(unused)]

fn main() {
    let x: Option<i32> = Some(5);
    let v: i32 = match x {
        Some(val) => val,
        None => panic!("None"),
    };

    // Unwraps the inner value. Panics if None
    let i = x.unwrap();
    println!("i: {}", i);

    let x: Result<i32, String> = Ok(5);
    let v: i32 = match x {
        Ok(val) => val,
        Err(e) => panic!("Error: {:?}", e),
    };

    // let x: Result<i32, String> = Err("Error".to_string());
    let i = x.unwrap();
    println!("result: {}", i);

    /*
    let x: Result<i32, String> = Err("Soemthing failed".to_string());
    let v: i32 = match x {
        Ok(val) => val,
        Err(err) => panic!("this is the error message: {:?}", err)
    };
    */

    let x: Result<i32, String> = Err("Something failed".to_string());
    x.expect("Something failed");
}