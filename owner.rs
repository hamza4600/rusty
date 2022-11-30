// concept of ownership in Rust  Memory safety is achieved by tight control on who can use what and when restrictions.

fn display(v: Vec<i32>) {
    println!("Vector is {:?}", v);
}

fn owner() {
    let v = vec![1, 2, 3, 4, 5]; //  v is owner of vector  vec is used to create vector in a heap
    let v2 = v; // v2 is owner of vector v
    display(v2); // v2 is passed to function display
                 // println!("Vector is {:?}", v); // v is not available here it has moved to v2
}

// retrun from a function
fn better(v: Vec<i32>) -> Vec<i32> {
    return v;
}

// concept of Borrowing
fn print_vector(v: &Vec<i32>) {
    println!("Vector is {:?}", v);
}

fn borrow() {
    let vec = vec![10, 20, 30, 40, 50];
    print_vector(&vec); // pass by reference  if dont pass by reference then it will give error
    println!("Pass vale By referience  {}", vec[0]);
}

//  Mutating Borrowing Values

fn add_one(e: &mut i32) {
    *e += 10;
}

fn mut_borrow() {
    let mut ii = 10; // it is mutable
    add_one(&mut ii);
    println!("Value of ii is {}", ii);
}

fn mut_str(name: &mut String) {
    println!("Name is {}", name);
    name.push_str(" Singh"); // change acctual value
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("s1 is {}", s1); // show erro
    println!("s2 is {}", s2);

    owner();
    let v = vec![1, 2, 3, 4, 5];
    let v1 = v;
    let v2_retrun = better(v1);
    println!("Vector is {:?}", v2_retrun);
    borrow();
    mut_borrow();

    let mut name = String::from("Hamza turing");
    mut_str(&mut name);
    println!("Original Name is {}", name);
}
