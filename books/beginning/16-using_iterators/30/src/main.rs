fn main() {
    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.into_iter().map(|x| x * 2) {
        print!("{} ", n);
    }
}
