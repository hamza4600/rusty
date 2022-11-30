// tuple and array
fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple is {:?}", tup);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of x is: {}", x);
}

fn array() {
    // methods to create array
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // let variable_name:[dataType;size] = [default_value_for_elements,size];
    println!("Array A is {:?}", a);
    println!("Array B is {:?}", b);
    println!("Arrat length is {}", a.len());

    let dummy: [i32; 14] = [-1; 14];
    println!("Array dummy is {:?}", dummy);
}

fn show_array() {
    let practice: [i32; 5] = [10, 20, 30, 40, 50];
    for val in practice.iter() {
        println!("Value is {}", val);
    }
}
//  pass array to function by reference and edit it
fn array_by_ref(arr: &mut [i32; 5]) {
    for val in 0..arr.len() {
        arr[val] = arr[val] + 1;
    }
    println!("Array is {:?}", arr);
}
// pass array to function by value and edit it
fn array_by_val(mut arr: [i32; 5]) {
    for val in 0..arr.len() {
        arr[val] = arr[val] + 10;
    }
    println!("Array is {:?}", arr);
}

fn problum() {
    const N : usize = 5;
    let mut arr: [i32; N] = [0; N];
    for i in 0..N {
        arr[i] = i as i32;
    }
    println!("Array is {:?}", arr);
}

fn main() {
    // tuple  can also access by index
    tuple();
    array();
    show_array();
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    array_by_ref(&mut arr);  // pass by reference
    array_by_val(arr); // pass by value
    problum();
}
