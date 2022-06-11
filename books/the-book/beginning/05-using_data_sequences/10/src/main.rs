fn main() {
    let x = ["a"];
    #[warn(unconditional_panic)]
    let _y = x[1];
    print!("End");
}
