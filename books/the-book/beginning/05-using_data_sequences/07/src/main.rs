fn main() {
    #[deny(unused_variables)]
    let x = 1;
    #[warn(unused_variables)]
    let y = 2;
    #[allow(unused_variables)]
    let z = 3;
}
