fn main() {
    let arr = [36, 1, 15, 9, 4];
    let v = arr.into_iter().collect::<Vec<_>>();
    print!("{:?}", v);
}
