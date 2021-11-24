use std::io;

fn floating_point_types() {
    let x = 2.; // f64
    let y: f32 = 3.0; //f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn the_boolean_type() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}

fn the_character_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn the_tuple_type_1() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn the_tuple_type_2() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}

fn the_tuple_type_2() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

fn the_array_type_1() {
    let a = [1, 2, 3, 4, 5];
}

fn the_array_type_2() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}

fn the_array_type_3() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}

fn the_array_type_4() {
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}

fn invalid_array_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn main() {
    invalid_array_element_access();
}
