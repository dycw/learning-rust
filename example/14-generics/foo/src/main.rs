use std::fs::create_dir;

use clap::Parser;
use inflector::Inflector;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    chapter: u64,
    // #[clap(short, long)]
    // names: Vec<String>,
}

fn main() {
    let args = Args::parse();
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
    core(args.chapter, names);
}

fn core(chapter: u64, names: Vec<&str>) {
    let snake_names = names.iter().map(|n| n.to_snake_case()).collect::<Vec<_>>();
    let enum_names = (1..)
        .zip(snake_names)
        .map(|(i, n)| format!("mkdir {}.{}-{}", chapter, i, n))
        .collect::<Vec<_>>();
    let (oks, errors): (Vec<_>, Vec<_>) = enum_names
        .iter()
        .map(|n| {
            println!("Making directory: {}", n);
            create_dir(n)
        })
        .partition(Result::is_ok);
    println!("{:?}", oks);
    println!("{:?}", errors);
}
