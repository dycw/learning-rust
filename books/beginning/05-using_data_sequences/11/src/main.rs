fn main() {
    let x = ["a"];
    #[allow(unconditional_panic)]
    let _y = x[1];
    print!("End");
}
