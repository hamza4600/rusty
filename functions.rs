// functions in rust are declared using the fn keyword
// When you pass parameters by reference, unlike value parameters, a new storage location is not created for these parameters.

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn display(s: String) {  // s: &String or s: &str
    println!("{}", s);
}

fn main() {
    let no: i32 = 5;
    mutate_no_to_zero(no);
    println!("The value of no is:{}", no);

    // let name: String = String::form("Hamza");
    // name.to_string();
    let a = 10;
    let b = 20;
    let c = add(a, b);
    println!("The value of c is:{}", c);

    let s = "Hello World";
    display(s.to_string());

}

fn mutate_no_to_zero(mut param_no: i32) {
    param_no = param_no * 0;
    println!("param_no value is :{}", param_no);
}
