fn main() {
    let s = "abc012è€";
    for i in 0..s.len() {
        println!("{}: {}", i, s.as_bytes()[i]);
    }
}
