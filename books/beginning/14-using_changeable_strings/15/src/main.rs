fn main() {
    let vs = ["Hello", ", ", "world", "!"];
    let mut result = String::new();
    for s in vs {
        result.push_str(s);
    }
    print!("{}", result);
}
