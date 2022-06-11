fn main() {
    use std::io::Write;
    let mut file = std::fs::File::create("data.txt").unwrap();
    file.write_all("eè€".as_bytes()).unwrap();
}
