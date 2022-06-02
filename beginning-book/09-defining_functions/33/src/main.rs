fn main() {
    let a = 15;
    let ref_a = &a;
    print!("{} {} {}", a, *ref_a, ref_a);
}
