fn main() {
    use std::mem::*;
    print!(
        "{} {} {} {}",
        size_of::<isize>(),
        size_of::<usize>(),
        size_of::<&i8>(),
        size_of::<&u32>()
    );
}
