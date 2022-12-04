// Basic  data structures

#![allow(dead_code)]

//  delete file
use std::fs::remove_file;

//  import LinkList
mod array;
mod bits;
mod linklist;
mod loops;
mod recursion;
mod problums;

fn remove(name: &str) {
    let path = name;
    let result = remove_file(path);
    match result {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => println!("Error deleting file: {}", e),
    }
}

fn main() {
    // remove("main"); // pass name of file to be deleted

    // let mut ll = linklist::LinkedList::new();
    // ll.append(10);
    // ll.append(20);
    // ll.append(30);
    // ll.append(40);
    // ll.show_node();
    // delete node
    // ll.delete(10);
    // print!("\n");
    // ll.show_node();

    // array
    // array::array();
    // array::array_vector();

    // recursion
    let aa = recursion::sum_to_n(500);
    println!("sum = {}", aa);
    let bb = recursion::factorial(5);
    println!("factorial = {}", bb);
    recursion::print_n(5);
    // let cc = recursion::fibo(8);
    // println!("fibo = {}", cc);
    // let dd = recursion::fibo_memo(8);
    // println!("fibo_memo = {}", dd);

    // Loops
    loops::loops_for(10);
    loops::table(50);
    // loops::loops_while(10);
    // loops::loops_loop(10);
    // loops::loops_array();
    // loops::loops_vector();
    // loops::doubel();
    // loops::loops_range_step();  // it will print 0 to 10 with step 2
    // loops::loops_range(); // it will print 0 to 10
    // loops::loops_string();

    // bits
    // let aa = bits::double(100);
    // println!("double = {}", aa);
    // let bb = bits::is_even(10100+100);
    // println!("is_even = {}", bb);
    // let cc = bits::swap(10, 20);
    // println!("swap = {:?}", cc);
    // let dd = bits::swap2(10, 20);
    // println!("swap2 = {:?}", dd);
    // let ee = bits::powerof_two(-10);
    // println!("powerof_two = {}", ee);
    // let ff = bits::set_bit(10, 2);
    // println!("set_bit = {}", ff);
    // let gg = bits::clear_bit(10, 2);
    // println!("clear_bit = {}", gg);

    // problums
    // problums::even_odd(10);
    problums::prime(100);    

}
