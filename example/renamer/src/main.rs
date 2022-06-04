use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs::{self, DirEntry};

fn main() -> Result<(), Box<dyn Error>> {
    let paths = std::fs::read_dir(".").unwrap();
    for path in paths.into_iter() {
        let x = path?;
        println!("{:?}", foo(&x))
    }
    Ok(())
}

#[derive(Debug)]
struct Rename {
    old: String,
    new: String,
}

fn foo(dir: &DirEntry) -> Option<String> {
    println!("{:?}", dir);
    if !dir.file_type().unwrap().is_dir() {
        return None;
    }
    let name = dir.file_name().into_string().unwrap();
    println!("{:?}", name);
    let re = Regex::new(r#"^main-(\d+)$"#).unwrap();
    let matches = re.captures_iter(&name).filter_map(|cap| cap.get(1));
    println!("{:?}", matches);
    Some("asdf".to_owned())
}
