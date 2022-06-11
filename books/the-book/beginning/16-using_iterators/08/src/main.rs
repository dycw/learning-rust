fn main() {
    let string: &str = "€èe";
    let string_it: std::str::Bytes = string.bytes();
    for byte in string_it {
        print!("{} ", byte);
    }
}
