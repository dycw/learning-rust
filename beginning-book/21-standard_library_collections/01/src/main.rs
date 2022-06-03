fn main() {
    let start_time = std::time::Instant::now();
    for i in 0..10_000 {
        println!("{}", i);
    }
    println!("{:?}", start_time.elapsed());
}
