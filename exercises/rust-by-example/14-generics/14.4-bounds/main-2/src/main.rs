use std::fmt::Display;

fn main() {
    struct S<T: Display>(T);

    // Error! `Vec<T>` does not implement `Display`. This
    // specialization will fail.
    let s = S(vec![1]);
}
