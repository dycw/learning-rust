fn main() {
    let mut v = vec![11, 22, 33];
    for _ in 0..v.len() {
        print!("{}, ", v.pop().unwrap())
    }
}
