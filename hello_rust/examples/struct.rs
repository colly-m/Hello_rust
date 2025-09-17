#![allow(unused)]

// Struct
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Point3D(i32, i32, i32);

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point: ({}, {})", p.x, p.y);
    print!("{:?}", p);

    let p = Point3D(10, 20, 30);
    println!("Point3D: ({}, {}, {})", p.0, p.1, p.2);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 0, y: 0 },
        radius: 5,
    };
    println!("Circle: {:#?}", circle);

    // Shortcut
    let x: i32 = 10;
    let y: i32 = 20;
    let p = Point { x: x, y: y };
    let p = Point { x, y };

    // Copy fields
    let p0 = Point { x: 1, y: 2, };
    let p1 = Point { x: 10, y: p0.y }; // y: 2
    let p1 = Point { x: 10, ..p0 }; // y:
    println!("p1 copy: {:?}", p1);
    // Update

    let mut p = Point { x: 1, y: 2 };
    p.x += 1;
    p.y += 1;
    println!("p update: {:?}", p);
}