fn main() {
    let mut n = 1;
    while true {
        let n2 = n * n;
        if n2 >= 200 {
            break;
        }
        print!("{} ", n2);
        n += 1;
    }
}
