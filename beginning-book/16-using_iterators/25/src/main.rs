fn main() {
    for item in vec![10, 20, 30].iter_mut() {
        *item += 1;
        print!("{} ", item);
    }
}
