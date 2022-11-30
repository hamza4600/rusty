// loops
fn main () {
    let mut x = 1;
    loop {
        x += 1;
        if x == 10 {
            break;
        }
    }
    println!("x is {}", x);
    // while loop
    let mut y = 1;
    while y != 10 {
        y += 1;
    }
    println!("y is {}", y);
    // for loop
    for z in 1..10 {
        println!("z is {}", z);
    }
    // for loop with array
    let array = [1, 2, 3, 4, 5];
    for i in array.iter() {
        println!("i is {}", i);
    }
    // for loop with range
    for i in (1..10).rev() {
        println!("i is {}", i);
    }


}