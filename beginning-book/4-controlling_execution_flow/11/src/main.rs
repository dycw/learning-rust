fn main() {
    let mut n = 0;
    while n < 50 {
        n += 1;
        if n % 3 != 0 {
            if n * n <= 400 {
                print!("{} ", n * n);
            }
        }
    }
}
