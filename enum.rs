// enum
// used to create custom data type have different values and properties
#![allow(dead_code)] //  allow dead code
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}
enum Number {
    Zero,
    One,
    Two,
}
enum Olor {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
//  match expression is used to match the value of enum with the pattern and execute the code block  associated with that pattern
#[derive(Debug)]
enum GenderCategory {
    Male,
    Female,
    Other,
}
#[derive(Debug)] // derive is used to print the struct
struct Persion {
    name: String,
    gender: GenderCategory,
}
fn persio() {
    let p1 = Persion {
        name: String::from("Usamn Abxc"),
        gender: GenderCategory::Male,
    };
    let p2 = Persion {
        name: String::from("Alexia"),
        gender: GenderCategory::Female,
    };
    println!("Persion 1 is {:?}", p1);
    println!("Persion 2 is {:?}", p2);
}

// match works like switch case
fn match_case() {
    let number = 4555;
    println!("Number is {}", number);
    match number {
        1 => println!("One"),
        2|3|5|7|11 => println!("Prime"),
        13..=19 => println!("Teen"),
        _ => println!("Not match"),
    }

    let boolen = true;
    let binary = match boolen {
        false => 0,
        true => 1,
    };
    println!("Binary is {}", binary);
}
fn main() {
    let c: Color = Color::Red;
    match c {
        // match expression
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RgbColor(0, 0, 0) => println!("Black"),
        Color::RgbColor(0, 0, 255) => println!("Blue"),
        Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("Black"),
        _ => println!("Not match"),
    }

    let n: Number = Number::One;
    match n {
        Number::Zero => println!("Zero"),
        Number::One => println!("One"),
        Number::Two => println!("Two"),
    }

    let o: Olor = Olor::Blue;
    match o {
        Olor::Red => println!("Red"),
        Olor::Green => println!("Green"),
        Olor::Blue => println!("Blue"),
    }

    println!("zero is {}", Number::Zero as i32);

    persio();
    match_case();
}
