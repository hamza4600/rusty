// recusrions.rs

//  sum to n
pub fn sum_to_n(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n + sum_to_n(n - 1)
    }
}

//  factorial
pub fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// print to n;
pub fn print_n(n: i32) {
    if n == 0 {
        println!("0");
    } else {
        print_n(n - 1);
        println!("{}", n);
    }
}
// fibo
pub fn fibo(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
//  optimized fibo by memoization
//  memoization is a technique to store the result of a function call
pub fn fibo_memo(n: i32) -> i32 {
    let mut memo: Vec<i32> = vec![0; (n + 1) as usize];
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        if memo[n as usize] != 0 {
            memo[n as usize]
        } else {
            memo[n as usize] = fibo_memo(n - 1) + fibo_memo(n - 2);
            memo[n as usize]
        }
    }
}

