fn main() {
    use std::mem::*;
    print!("{} ", size_of::<i32>());
    print!("{} ", size_of_val(&12));
}
