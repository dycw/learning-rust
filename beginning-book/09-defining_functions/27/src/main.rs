fn main() {
    fn double(mut a: [i32; 10]) {
        for i in 0..10 {
            a[i] *= 2;
        }
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double(arr);
    print!("{:?}", arr);
}
