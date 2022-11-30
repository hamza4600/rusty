fn main() {
    let company = "Turing is Great";
    let rigth_float = 4.56;
    let is_good = true;
    let icons = '🦀';
    println!(
        "company name {} float {} boolen {} icons {}",
        company, rigth_float, is_good, icons
    );
    let num = 456; // i32 by default;
    let small: i32 = 450;
    let mark: isize = 5000;
    let big: usize = 5000;
    println!("num {} small {} mark {} big {}", num, small, mark, big);
    // let custom_type = CustomType { //  custom type
    //     name: "Hamza".to_string(),
    //     age: 20,
    //     height: 5.5,
    // };

    // by defalut from 0 to 255 only allowd in u8
    // let byte:u8 = 255; // if is 256 then it will give error
    //  by default from -128 to 127 only allowd in i8

    // Floats
    let float_one = 1.0; // f64 by default
    let float_two: f32 = 1.0; // f32
    let float_three: f64 = 1526.15620; // f64
    println!(
        "foats value are {} and {} and {}",
        float_one, float_two, float_three
    );

    // Automatic type casting is not allowed in Rust.
    // let aba:f32 = 10;  //  error

    // large number
    let large_num = 1_000_000_000; // 1 billion
    let large_num1: i64 = 1_000_000_000_000; // 1 trillion
    println!(
        "large number is {} and very large {}\n",
        large_num, large_num1
    );

    //  strings
    let string_one = "Hello World";
    let string_two = "Hello World".to_string();
    println!("string one {} and string two {}", string_one, string_two);

    let mut my_fees = 456163;
    println!("my fees is {}", my_fees);
    my_fees = 456163780;
    println!("my fees is {}", my_fees);

    // let and const
    let my_name = "Hamza";
    let my_name = my_name.len(); // always used second value if same name is used

    println!("my name is length is  {}", my_name);

    const NAME: &str = "Johan Smith"; // START WITH CAPITAL LETTERS AND ALWAYS USE &str OR data type
    const LENGTH: usize = NAME.len();
    println!("name is {} and length is {}", NAME, LENGTH);
}

// println!  two arrguments {}  is a placeholder for the value and name or constant
