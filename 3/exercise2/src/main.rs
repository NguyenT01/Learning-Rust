// EXERCISE 2: NTH FIBONACCI
use std::io;

fn main() {
    println!("Enter n-th fibonacci number");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number");

    let result = fibonacci(n);
    println!("f({n}) = {result}");

    let result2 = fibonacci_loop(n);
    println!("f({n}) = {}", result2);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn fibonacci_loop(n: u32) -> u32 {
    let mut result: u32 = 0;
    let mut prev: u32=0;

    for i in 0..=n {
        if i == 1 {
            prev = 0;
            result = 1;
        } else {
            let tmp = result;
            result += prev;
            prev = tmp;
        }
    }
    result
}
