#![allow(unused)]

// Question operator - ?

fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(1)
}

fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}


fn f1_f2_match() -> Result<u32, String> {
    let x = f1();
    let y = match x {
        Ok(val) => val,
        Err(err) => {
            return Err("err from f1".to_string());
        }
    };
    let z = f2();
    let w = match z {
        Ok(val) => val,
        Err(err) => {
            return Err("err from f2".to_string());
        }
    };
    Ok(y + w)
}

fn f1_f2_question() -> Result<u32, String> {
    let x = f1()?;
    let y = f2()?;
    Ok(x + y)
}

fn main() {
    let x = f1_f2_question();
    println!("x: {:?}", x);
}