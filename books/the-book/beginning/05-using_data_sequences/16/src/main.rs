fn main() {
    let mut fib = [1; 12];
    for i in 2..fib.len() {
        fib[i] = fib[i - 2] + fib[i - 1];
    }
    for i in 0..fib.len() {
        print!("{}, ", fib[i]);
    }
}
