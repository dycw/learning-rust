fn main() {
    let mut v = vec![10, 20, 30];
    for item in v.into_iter() {
        item += 1; // ILLEGAL: item is not mutable
        print!("{} ", item);
    }
}
