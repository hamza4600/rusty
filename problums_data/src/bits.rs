// bist amupulation

pub fn double(n: i32) -> i32 {
    n << 1
}

pub fn is_even(n: i32) -> bool {
    n & 1 == 0
}

pub fn swap(n: i32, m: i32) -> (i32, i32) {
    (m, n)
}

pub fn swap2(n: i32, m: i32) -> (i32, i32) {
    (n ^ m, n ^ m ^ n)
}

pub fn powerof_two(n: i32) -> i32 {
    n & (n - 1) // it will tell us if the number is power of 2 or not
}

pub fn set_bit(n: i32, i: u32) -> i32 {
    n | (1 << i)
}

pub fn clear_bit(n: i32, i: u32) -> i32 {
    n & !(1 << i)
}

pub fn right_shift(n: i32, i: u32) -> i32 {
    n >> i
}

