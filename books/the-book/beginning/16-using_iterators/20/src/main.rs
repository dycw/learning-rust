fn main() {
    let v = vec![3, 4, 5];
    let iterator: std::slice::IterMut<i32> = v.iter_mut();
    for mut_item_ref in iterator {
        *mut_item_ref += 1;
    }
    print!("{:?}", v);
}
