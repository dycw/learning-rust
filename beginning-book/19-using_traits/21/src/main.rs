fn main() {
    impl str {
        // ILLEGAL: Inherent implementation for str type
        fn length(&self) -> usize {
            self.chars().count()
        }
    }
    let s = "€èe";
    print!("{} {}", s.len(), s.length());
}
