fn main() {
    let mut x = vec!["This", "is", "a", "sentence"];
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.insert(1, "line");
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.insert(2, "contains");
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.remove(3);
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.push("about Rust");
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.pop();
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
}
