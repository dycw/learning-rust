use std::ops::{self, Mul};

// implement fn multiply to make the code work
// As mentiond above, `+` needs `T` to implement `std::ops::Add` Trait
// so, what about `*` ?  You can find the answer here: https://doc.rust-lang.org/core/ops/
fn multiply<T>(x: T, y: T) -> T
where
    T: Mul + Mul<Output = T>,
{
    x * y
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!")
}
