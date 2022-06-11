use std::ops::Add;

fn main() {
    assert_eq!(sum(1, 2), 3);
}

// implement `fn sum` with trait bound in two ways
fn sum<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}
