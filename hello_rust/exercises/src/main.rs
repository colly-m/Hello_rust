#![allow(unused)]

fn main() {
    // Modify to fix s1 and s2 to reference to s
    let s = String::from("Rust");
    let s1 = &mut s;
    let s2 = &mut s;


    // Modify the function of print_len so the code compiles
    fn print_len(s: &String) {
        println!("Length: {}", s.len());
    }

}