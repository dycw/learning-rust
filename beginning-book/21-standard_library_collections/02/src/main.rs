fn main() {
    use std::time::Instant;
    const SIZE: usize = 100_000_000;
    let start_time = Instant::now();
    let mut v = Vec::<usize>::with_capacity(SIZE);
    let t1 = start_time.elapsed();
    for i in 0..SIZE {
        v.push(i);
    }
    let t2 = start_time.elapsed();
    for _ in 0..SIZE {
        v.pop();
    }
    let t3 = start_time.elapsed();
    print!("{:?} {:?} {:?}", t1, t2 - t1, t3 - t2);
}
