fn main() {
    let arr = ['a', 'b', 'c'];
    for (index, ch) in arr.into_iter().enumerate() {
        print!("{} {}, ", index, ch);
    }
}
