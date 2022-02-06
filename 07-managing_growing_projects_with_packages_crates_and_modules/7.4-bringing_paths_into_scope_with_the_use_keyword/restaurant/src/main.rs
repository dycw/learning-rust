use std::collections::HashMap;

// Creating Idiomatic use Paths

fn creating_idiomatic_use_paths() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn using_external_packages() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}

// Using Nested Paths to Clean Up Large use Lists

use std::cmp::Ordering;
use std::io;
use std::{cmp::Ordering, io};

fn main() {
    using_external_packages();
}
