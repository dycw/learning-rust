fn main() {
    let s = "Hello, world!";
    let ch = 'R';
    let mut contains = false;
    for c in s.chars() {
        if c == ch {
            contains = true;
        }
    }
    print!(
        "\"{}\" {} '{}'.",
        s,
        if contains {
            "contains"
        } else {
            "does not contain"
        },
        ch
    );
}
