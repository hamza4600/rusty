// Opertaions in the game

fn main() {
    // 1. Addition
    let sum = 5 + 10;
    println!("Sum is {}", sum);

    // comparison
    let is_greater = 10 > 5;
    println!("Is greater {}", is_greater);

    // conditional
    let a = 10;
    let b = 20;
    let c = if a > b {
        a
    } else {
        b
    };
    println!("c is {}", c);

    // Bitwise
    let two_to_the_10 = 1 << 10;
    println!("2^10 is {}", two_to_the_10);

    // OR
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("PI less than 4 {}", pi_less_4);

    let num = 26;
    if num % 2 == 0 {  // in Bits is (num & 1 == 0)
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }


}