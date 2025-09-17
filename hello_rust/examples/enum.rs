#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Pause,
    Stop,
    Skip(u32),
    Rewind(u32),
    Resize { width: u32, height: u32 },
}

// Enum
fn main() {
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize { width: 1920, height: 1080 };

    println!("{:?}", cmd);

    // PartialEq
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("{:?}", cmd0 == cmd1);

    // Option<T> = Some(T) | None
    let x: Option<i32> = Some(5);
    let x: Option<i32> = None;
    
    // Result<T, E> = Ok(T) | Err(E)
    // "100" -> Ok(100)
    let y: Result<i32, String> = Ok(5);
    // "abc" -> Err("Not a number")
    let y: Result<i32, String> = Err("Not a number".to_string());
    println!("{:?}", y);

}