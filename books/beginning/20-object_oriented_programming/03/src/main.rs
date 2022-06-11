fn main() {
    struct S(i32);
    impl Drop for S {
        fn drop(&mut self) {
            println!("Dropped {}", self.0);
        }
    }
    let _ = S(1);
    let _ = S(2);
    let _ = S(3);
    {
        let _ = S(4);
        let _ = S(5);
        let _ = S(6);
        println!("INNER");
    }
    println!("OUTER");
}
