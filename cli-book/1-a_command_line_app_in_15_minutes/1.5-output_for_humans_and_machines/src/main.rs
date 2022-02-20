// Printing “Hello World”

fn printing_hello_world() {
    println!("Hello World");
}

// Using println

fn using_println_1() {
    let x = 42;
    println!("My lucky number is {}.", x);
}

fn using_println_2() {
    let xs = vec![1, 2, 3];
    println!("The list is: {:?}", xs);
}

// Printing errors

fn printing_errors() {
    println!("This is information");
    eprintln!("This is an error! :(");
}

// A note on printing performance

use std::io::{self, Write};

fn a_note_on_printing_performance() {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
}

// Showing a progress bar

use std::{thread, time::Duration};

fn showing_a_progress_bar() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        thread::sleep(Duration::from_millis(10));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

// Logging

use env_logger;
use log::{info, warn};

fn logging() {
    // run as:
    //     env RUST_LOG=info cargo run
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}

fn main() {
    logging();
}
