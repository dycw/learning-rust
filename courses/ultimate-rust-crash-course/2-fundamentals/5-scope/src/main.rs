fn main() {
    let x = 5;
    {
        let x = 99;
        println!("{}", x);
    }
    println!("{}", x);
}
