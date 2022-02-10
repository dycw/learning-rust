// Recoverable Errors with Result

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

fn matching_on_different_errors() {
    let f = File::open("matching_on_different_errors.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("matching_on_different_errors.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn main() {
    matching_on_different_errors();
}
