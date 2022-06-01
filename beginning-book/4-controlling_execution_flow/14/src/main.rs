fn main() {
    let mut n = 1;
    loop {
        let nn = n * n;
        if nn >= 200 {
            break;
        }
        print!("{} ", nn);
        n += 1;
    }
}
