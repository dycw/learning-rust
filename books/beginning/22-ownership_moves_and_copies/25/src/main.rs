fn main() {
    struct S {}
    impl Clone for S {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    let s = S {};
    let _ = s.clone();
    let _s2 = s;
    let _s3 = s;
}
