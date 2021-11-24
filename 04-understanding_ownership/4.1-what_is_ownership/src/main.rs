fn variable_scope() {
    let s = "hello";
}

fn the_string_type_1() {
    let s = String::from("hello");
}

fn the_string_type_2() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn ways_variables_and_data_interact_move() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
}

fn ways_variables_and_data_interact_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_only_data_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y= {}", x, y)
}

fn ownership_and_functions() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn return_values_and_scope_1() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn return_values_and_scope_2() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len)
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    return_values_and_scope_2();
}
