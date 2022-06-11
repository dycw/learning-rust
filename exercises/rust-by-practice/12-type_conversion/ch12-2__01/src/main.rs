fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    // 1. impl From<char> for ? , maybe you should check the docs mentiond above to find the answer
    // 2. a keyword from the last chapter
    let i3: u32 = 'a'.into();

    // FIX the error in two ways
    let s: String = 'a'.into();

    println!("Success!")
}
