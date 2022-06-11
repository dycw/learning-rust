fn main() {
    fn f(n: &u8) -> &'static u8 {
        n
    }
    fn g<'a>(m: &'a u8) -> &'static u8 {
        m
    }
}
