//

pub fn even_odd(n: i32) {
    let mut odd = Vec::new();
    let mut even = Vec::new();
    for i in 0..n {
        if i % 2 == 0 {
            even.push(i);
        } else {
            odd.push(i);
        }
    }
    println!("Even numbers are: ");
    for i in even.iter() {
        println!("i = {}", i);
    }
    println!("Odd numbers are: ");
    for i in odd.iter() {
        println!("i = {}", i);
    }
}

pub fn prime(n:i32) {
    let mut prime = Vec::new();
    for i in 2..n {
        let mut flag = true;
        for j in 2..i {
            if i % j == 0 {
                flag = false;
                break;
            }
        }
        if flag {
            prime.push(i);
        }
    }
    // println!("Prime numbers are: ");
    // for i in prime.iter() {
        // println!("i = {}", i);
    // }
    // or full arry pirnt
    print!("Array is : {:?} \n", prime);
}
