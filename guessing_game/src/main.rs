fn fib(n: u32) -> u32 {
    if n <= 0 {
        1
    } else {
        n * fib(n - 1)
    }
}

fn main() {
    println!("fib({}) = {}", 5, fib(5))
}
