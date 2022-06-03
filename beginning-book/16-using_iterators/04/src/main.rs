fn main() {
    fn print_codes(s: &str) {
        for c in s.chars() {
            println!("{}: {}", c, c as u32);
        }
    }
    print_codes("€èe");
}
