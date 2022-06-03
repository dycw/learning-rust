fn main() {
    let mut line = String::new();
    println!("{:?}", std::io::stdin().read_line(&mut line));
    println!("[{}]", line);
}
