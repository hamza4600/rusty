// insertt data into hash table

use std::collections::HashMap;

fn main () {
    let mut hash = HashMap::new();
    hash.insert("Rust", "Programming Language");
    hash.insert("Python", "Programming Language");
    hash.insert("Java", "Programming Language");
    hash.insert("C++", "Programming Language");
    hash.insert("C", "Programming Language");
    hash.insert("C#", "Programming Language");
    hash.insert("JavaScript", "Programming Language");
    hash.insert("PHP", "Programming Language");

    println!("{:?}", hash);

    if hash.contains_key("Rust") {
        println!("Rust is a programming language found in hash table");
    }
}