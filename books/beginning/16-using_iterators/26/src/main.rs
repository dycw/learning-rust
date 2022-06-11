fn main() {
    for item in &mut vec![10, 20, 30] {
        *item += 1;
        print!("{} ", item);
    }
}
