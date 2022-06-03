fn main() {
    let mut s1 = "".to_string();
    for _ in 0..16 {
        println!("{:p} {} {}", s1.as_ptr(), s1.capacity(), s1.len());
        s1.push('a');
    }
    let s2 = "x".to_string();
    s1.push('-');
    println!("{:p}", s2.as_ptr());
    println!("{:p} {} {}: {}", s1.as_ptr(), s1.capacity(), s1.len(), s1);
}
