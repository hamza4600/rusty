// Basic  data structures

#![allow(dead_code)]

//  delete file
use std::fs::remove_file;

//  import LinkList
mod linklist;

fn remove(name: &str) {
    let path = name;
    let result = remove_file(path);
    match result {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => println!("Error deleting file: {}", e),
    }
}

fn arr() {
    // vectro
    let mut v: Vec<i32> = Vec::new(); // array can gro and shrink
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("v = {:?}", v);

    // 2nd way
    let vv = vec![10, 20, 30, 40];
    println!("v = {:?}", vv);
    //  search in vector
    let i = vv[2]; // give value at index 2
    println!("i = {}", i);
    //  search in vector
    let i = vv.get(2); // give value at index 2
    println!("i = {:?}", i);

    // 3rd way
    // let mut vvv = Vec::with_capacity(10);
    // vvv.push(100);
    // vvv.push(200);
    // vvv.push(300);
    // vvv.push(400);
    // println!("v = {:?/}", vvv);
}

fn recursion() {
    fn factorial(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
    println!("factorial of 5 is {}", factorial(5));
}

fn main() {
    arr();
    remove("main"); // pass name of file to be deleted

    let mut ll = linklist::LinkedList::new();
    ll.append(10);
    ll.append(20);
    ll.append(30);
    ll.append(40);
    // ll.show_node();
    // delete node
    ll.delete(10);
    print!("\n");
    ll.show_node();
    // rescusrion
    recursion();
}
