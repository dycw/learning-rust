use std::fmt::Display;

// The Borrow Checker

fn the_borrow_checker() {
    let x = 5; //            ----------+-- 'b
               //                      |
    let r = &x; //           --+-- 'a  |
                //             |       |
    println!("r: {}", r); //   |       |
                          // --+       |
} //                         ----------+

// Lifetime Annotations in Function Signatures

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_annotations_in_function_signatures_1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn lifetime_annotations_in_function_signatures_2() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// Lifetime Annotations in Struct Definitions

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_annotations_in_struct_definitions() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Lifetime Elision

fn first_word_1(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.into_iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_2<'a>(s: &'a str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.into_iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.into_iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Lifetime Annotations in Method Definitions

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The Static Lifetime

fn the_static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    lifetime_annotations_in_struct_definitions();
}
