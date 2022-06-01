fn main() {
    let mut limit = 4;
    for n in 1..limit + 2 {
        limit -= 1;
        print!("{} {}, ", limit, n);
    }
    print!("{}", limit);
}
