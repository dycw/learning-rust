// Creating a New String

fn creating_a_new_string_1() {
    let mut s = String::new();
}

fn creating_a_new_string_2() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn creating_a_new_string_3() {
    let s = String::from("initial contents");
}

fn creating_a_new_string_4() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

// Appending to a String with push_str and push

fn appending_to_a_string_with_push_str_and_push_1() {
    let mut s = String::from("foo");
    s.push_str("bar");
}

fn appending_to_a_string_with_push_str_and_push_2() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2)
}
fn appending_to_a_string_with_push_str_and_push_3() {
    let mut s = String::from("lo");
    s.push_str("l");
}

// Concatenation with the + Operator or the format! Macro

fn concatenation_with_the_plus_operator_or_the_format_macro_1() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
}

fn concatenation_with_the_plus_operator_or_the_format_macro_2() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
}

fn concatenation_with_the_plus_operator_or_the_format_macro_3() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}

// Internal Representation

fn internal_representation_1() {
    let hello = String::from("Hola");
}

fn internal_representation_2() {
    let hello = String::from("Здравствуйте");
}

// Slicing Strings

fn slicing_strings() {
    let hello = "Здравствуйте";

    let s = &hello[0..1];
}

// Methods for Iterating Over Strings

fn methods_for_iterating_over_strings_1() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

fn methods_for_iterating_over_strings_2() {
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
fn main() {
    methods_for_iterating_over_strings_2();
}
