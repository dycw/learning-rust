// Recoverable Errors with Result

use std::fs::{self, File};

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

// Shortcuts for Panic on Error: unwrap and expect

fn shortcuts_for_panic_on_error_unwrap_and_expect_1() {
    let f = File::open("shortcuts_for_panic_on_error_unwrap_and_expect_1.txt").unwrap();
}

fn shortcuts_for_panic_on_error_unwrap_and_expect_2() {
    let f = File::open("shortcuts_for_panic_on_error_unwrap_and_expect_2.txt")
        .expect("Failed to open shortcuts_for_panic_on_error_unwrap_and_expect_2.txt");
}

// Propagating Errors

use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("read_username_from_file.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator

fn a_shortcut_for_propagating_errors_the_question_mark_operator_1() -> Result<String, io::Error> {
    let mut f = File::open("a_shortcut_for_propagating_errors_the_question_mark_operator_1.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn a_shortcut_for_propagating_errors_the_question_mark_operator_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("a_shortcut_for_propagating_errors_the_question_mark_operator_2.txt")?
        .read_to_string(&mut s)?;

    Ok(s)
}

fn a_shortcut_for_propagating_errors_the_question_mark_operator_3() -> Result<String, io::Error> {
    fs::read_to_string("a_shortcut_for_propagating_errors_the_question_mark_operator_3.txt")
}

// Where The ? Operator Can Be Used

// fn where_the_question_mark_operator_can_be_used_1() {
//     let f = File::open("where_the_question_mark_operator_can_be_used.txt")?;
// }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

use std::error::Error;

fn where_the_question_mark_operator_can_be_used_2() -> Result<(), Box<dyn Error>> {
    let f = File::open("where_the_question_mark_operator_can_be_used_2.txt")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    where_the_question_mark_operator_can_be_used_2()
}
