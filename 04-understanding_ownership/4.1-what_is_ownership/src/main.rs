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

fn memory_and_allocation() {
    let s = String::from("hello");
}

fn ways_variables_and_data_interact_move_1() {
    let x = 5;
    let y = x;
}

fn ways_variables_and_data_interact_move_2() {
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

    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn main_1() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn main_2() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len)
}
