// Recoverable Errors with Result

use core::panic;
use core::panicking::panic;
use std::fs::File;

fn recoverable_errors_with_result_1() {
    let f = File::open("recoverable_errors_with_result_1.txt");
}

fn recoverable_errors_with_result_2() {
    let f = File::open("recoverable_errors_with_result_2.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// Matching on Different Errors

use std::io::ErrorKind;

fn matching_on_different_errors_1() {
    let f = File::open("matching_on_different_errors_1.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("matching_on_different_errors_1.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn matching_on_different_errors_2() {
    let f = File::open("matching_on_different_errors_2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("matching_on_different_errors_2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}

fn main() {
    matching_on_different_errors_2();
}
