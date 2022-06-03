fn main() {
    fn print_codes(s: &str) {
        let mut iter = s.chars();
        loop {
            match iter.next() {
                Some(c) => {
                    println!("{}: {}", c, c as u32);
                }
                None => {
                    break;
                }
            }
        }
    }
    print_codes("€èe");
}
