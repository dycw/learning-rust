use std::fs::create_dir;

use clap::Parser;
use inflector::Inflector;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    chapter: String,

    #[clap(short, long)]
    names: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let names = vec![
        "derive",
        "returning traits with dyn",
        "operator overloading",
        "drop",
        "iterators",
        "impl trait",
        "clone",
        "supertraits",
        "disambiguating overlapping traits",
    ];
    core(&args.chapter, &names);
}

fn core(chapter: &str, names: &Vec<&str>) {
    let snake_names = names.iter().map(|n| n.to_snake_case()).collect::<Vec<_>>();
    let enum_names = (1..)
        .zip(snake_names)
        .map(|(i, n)| format!("{}.{}-{}", chapter, i, n))
        .collect::<Vec<_>>();
    let (oks, errors): (Vec<_>, Vec<_>) = enum_names
        .iter()
        .map(|n| {
            println!("Making directory: {}", n);
            create_dir(format!("../{}", n))
        })
        .partition(Result::is_ok);
    println!("{:?}", oks);
    println!("{:?}", errors);
}
