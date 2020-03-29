use std::env;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let fib_int = arg1.parse().unwrap();
        println!("{}", fib(fib_int))
    } else {
        println!("Missing Input")
    }
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
