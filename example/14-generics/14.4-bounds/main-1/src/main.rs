use std::fmt::Display;

fn main() {
    // Define a function `printer` that takes a generic type `T` which
    // must implement trait `Display`.
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }
}
