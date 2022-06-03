fn main() {
    struct LargeNumber(f64);
    struct SmallNumber(f32);
    let ln = LargeNumber(1. / 3.);
    let sn = ln as SmallNumber;
    print!("{}", sn.0);
}
