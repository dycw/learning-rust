// FILL in the blanks
fn main() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s; // in two ways
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");

    let mut slice3: String = s.clone();
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
}
