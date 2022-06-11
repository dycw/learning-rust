fn main() {
    {
        // First invocation of `print_sum`
        let addend1: f64 = 3.;
        let addend2: f64 = 5.;
        println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
    }
    {
        // Second invocation of `print_sum`
        let addend1: f64 = 3.2;
        let addend2: f64 = 5.1;
        println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
    }
}
