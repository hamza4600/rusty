// Loops
pub fn loops_for(n: i32) {
    for i in 0..n {
        println!("i = {}", i);
    }
}

// Table
pub fn table(n: i32) {
    print!("Tabel of {} is: ", n);
    println!("");
    for i in 1..11 {
        println!("{} * {} = {}", n, i, n * i);
    }
}

//  other loops
//  while
pub fn loops_while(n: i32) {
    let mut i = 0;
    while i < n {
        println!("i = {}", i);
        i += 1;
    }
}

//  loop
pub fn loops_loop(n: i32) {
    let mut i = 0;
    loop {
        println!("i = {}", i);
        i += 1;
        if i == n {
            break;
        }
    }
}

// loops and array

pub fn loops_array() {
    let arr = [10, 20, 30, 40, 50];
    for i in arr.iter() {
        println!("i = {}", i);
    }
}

// chnages and vectors

pub fn loops_vector() {
    let mut names = vec!["cbhf34", "c5e6r", "45x32"];
    names.push("378gex");
    for i in names.iter() {
        println!("i = {}", i);
    }
}

pub fn doubel() {
    let mut arr = [10, 20, 30, 40, 50];
    for i in arr.iter_mut() {
        *i *= 2;
    }
    for i in arr.iter() {
        println!("i = {}", i);
    }
}

//  loops and string
pub fn loops_string() {
    let name = "xb392whjs9N392X";
    for i in name.chars() {
        println!("i = {}", i);
    }
}

//  loops and range
pub fn loops_range() {
    for i in (1..10).rev() {
        println!("i = {}", i);
    }
}

//  loops and range
pub fn loops_range_step() {
    for i in (1..10).rev().step_by(2) {
        println!("i = {}", i);
    }
}

