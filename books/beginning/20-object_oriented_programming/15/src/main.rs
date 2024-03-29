fn main() {
    struct LargeNumber(f64);
    struct SmallNumber(f32);
    impl From<LargeNumber> for SmallNumber {
        fn from(source: LargeNumber) -> Self {
            Self(source.0 as f32)
        }
    }
    let ln = LargeNumber(1. / 3.);
    let sn: SmallNumber = ln.into();
    print!("{}", sn.0);
}
