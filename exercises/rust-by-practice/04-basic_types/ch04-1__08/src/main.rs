fn main() {
    assert!((((0.1 + 0.2) - 0.3) as f64).abs() <= 1e-16);

    println!("Success!");
}
