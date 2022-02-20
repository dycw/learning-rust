// The Slice Type

fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn the_slice_type_1() {
    let mut s = String::from("hello world");

    let word = first_word_1(&s);

    s.clear();
}

// String Slices

fn string_slices_1() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn string_slices_2() {
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
}

fn string_slices_3() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[..];
}

fn string_slices_4() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn the_slice_type_2() {
    let s = String::from("hello world");

    let word = first_word_2(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}

// String Literals Are Slices

fn string_literals_are_slices() {
    let s = "Hello, world!";
}

// String Slices as Parameters

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn the_slice_type_3() {
    let my_string = String::from("hello world");

    let word = first_word_3(&my_string[0..6]);
    let word = first_word_3(&my_string[..]);
    let word = first_word_3(&my_string);

    let my_string_literal = "hello world";

    let word = first_word_3(&my_string_literal[0..6]);
    let word = first_word_3(&my_string_literal[..]);

    let word = first_word_3(my_string_literal);
}

// Other Slices

fn other_slices_1() {
    let a = [1, 2, 3, 4, 5];
}

fn other_slices_2() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3])
}

fn main() {
    other_slices_2();
}
