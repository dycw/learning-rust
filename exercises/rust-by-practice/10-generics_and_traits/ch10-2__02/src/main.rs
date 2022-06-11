use std::fmt::Debug;

// fill in the blanks to make it work
fn print_array<T, const N: usize>(arr: [T; N])
where
    T: Debug,
{
    println!("{:?}", arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
