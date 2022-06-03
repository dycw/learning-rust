fn main() {
    let arr = ['a', 'b', 'c'];
    let mut index = 0;
    for ch in arr.into_iter() {
        print!("{} {}, ", index, ch);
        index += 1;
    }
}
