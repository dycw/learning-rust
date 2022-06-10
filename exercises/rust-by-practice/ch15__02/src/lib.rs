//! A library for showing how to use doc comments

pub mod compute;

/// Add one to the given value and return a new value
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ch15__02::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/** Add two to the given value and return a new value
# Examples
```
let arg = 5;
let answer = ch15__02::add_two(arg);
assert_eq!(7, answer);
```
*/
pub fn add_two(x: i32) -> i32 {
    x + 2
}
