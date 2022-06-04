

use inflector::{Inflector};

fn main() {
    let chapter = 14;
    let names = vec![
        "functions",
        "implementation",
        "traits",
        "bounds",
        "multiple bounds",
        "where clauses",
        "new type idiom",
        "associated items",
        "phantom type parameters",
    ];
    core(chapter, names);
}
fn core(chapter: u64, names: Vec<&str>) {
    let snake_names = names.iter().map(|n| n.to_snake_case()).collect::<Vec<_>>();
    let enum_names = (1..)
        .zip(snake_names)
        .map(|(i, n)| format!("mkdir {}.{}-{}", chapter, i, n))
        .collect::<Vec<_>>();
    enum_names
        .iter()
        .for_each(|n| println!("Making directory: {}", n));
}