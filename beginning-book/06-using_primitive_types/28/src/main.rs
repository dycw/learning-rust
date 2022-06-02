fn main() {
    for n in 32..127 {
        println!("{}: [{}]", n, n as u8 as char);
    }
    for n in 160..256 {
        println!("{}: [{}]", n, n as u8 as char);
    }
}
