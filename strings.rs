// string and its methods
// strings are of two types
//Strings Liters(&str)
// String Object(String)

// STRING LITERALS
// let string_one = "Hello World";
// hardcoded by default  are immutable and static in memory need to valid for whole program

// STRING OBJECT
// let string_two = "Hello World".to_string();
// created by using to_string() method and are mutable and dynamic in memory and haev alot od methods
//

fn main() {
    let empty_string = String::new();
    let string_one = "Hello World";
    let string_two = "Hello World Pakistan".to_string();
    let string_three = String::from("Hello World U S A");

    println!(
        "{} {} {} {}",
        empty_string, string_one, string_two, string_three
    );

    let string_methods = String::from("Dynamic in Memeory ");
    println!("Length of string is {}", string_methods.len());
    println!("Is string empty {}", string_methods.is_empty());
    println!("Contains string {}", string_methods.contains("Dynamic"));
    println!(
        "Replace string {}",
        string_methods.replace("Dynamic", "Static")
    );
    for word in string_methods.split_whitespace() {
        println!("loop {}", word);
    }
    // split() method is used to split string on a given character
    println!("Split {}", string_methods.split("@").next().unwrap());

    // trim() method is used to remove white spaces from string
    let mut anoter_way = String::new();
    anoter_way.push_str("I am Learning Rust");
    println!("Trim {}", anoter_way.trim());

    for i in anoter_way.chars() {
        println!("__{}", i);
    }
}
