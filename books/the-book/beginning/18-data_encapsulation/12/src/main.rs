fn main() {
    {
        fn f() -> u32 {
            g()
        }
        fn g() -> u32 {
            123
        }
    }
    print!("{}", f());
}
