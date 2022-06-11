// Pattern Syntax

fn pattern_syntax() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// Matching Named Variables

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);
}

// Multiple Patterns

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// Matching Ranges of Values with ..=

fn matching_ranges_of_values_with_dot_dot_equal_1() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn matching_ranges_of_values_with_dot_dot_equal_2() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// Destructuring Structs

struct Point1 {
    x: i32,
    y: i32,
}

fn destructuring_structs_1() {
    let p = Point1 { x: 0, y: 7 };

    let Point1 { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn destructuring_structs_2() {
    let p = Point1 { x: 0, y: 7 };

    let Point1 { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructuring_structs_3() {
    let p = Point1 { x: 0, y: 7 };

    match p {
        Point1 { x, y: 0 } => println!("On the x axis at {}", x),
        Point1 { x: 0, y } => println!("On the y axis at {}", y),
        Point1 { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// Destructuring Enums

enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums() {
    let msg = Message1::ChangeColor(0, 160, 255);

    match msg {
        Message1::Quit => println!("The Quit variant has no data to destructure"),

        Message1::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message1::Write(text) => println!("Text message: {}", text),
        Message1::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// Destructuring Nested Structs and Enums

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructuring_nested_structs_and_enums() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v,
            )
        }
        _ => (),
    }
}

// Destructuring Structs and Tuples

fn destructuring_structs_and_tuples() {
    let ((feet, inches), Point1 { x, y }) = ((3, 10), Point1 { x: 3, y: -10 });
}

// Ignoring an Entire Value with _

fn foo(_: i32, y: i32) {
    println!("This code only uses the y argument: {}", y);
}

fn ignoring_an_entire_value_with_underscore() {
    foo(3, 4)
}

// Ignoring Parts of a Value with a Nested _

fn ignoring_parts_of_a_value_with_a_nested_underscore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

// Ignoring an Unused Variable by Starting Its Name with _

fn ignoring_an_unused_variable_by_starting_its_name_with_underscore_1() {
    let _x = 5;
    let y = 10;
}

fn ignoring_an_unused_variable_by_starting_its_name_with_underscore_2() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

// Ignoring Remaining Parts of a Value with ..

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

fn ignoring_remaining_parts_of_a_value_with_dot_dot_1() {
    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }
}

fn ignoring_remaining_parts_of_a_value_with_dot_dot_2() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("usome numbers: {}, {}", first, last);
        }
    }
}

// Extra Conditionals with Match Guards

fn extra_conditionals_with_match_guards_1() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn extra_conditionals_with_match_guards_2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: {:?}, y = {}", x, y);
}

fn extra_conditionals_with_match_guards_3() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @ Bindings

enum Message3 {
    Hello { id: i32 },
}

fn at_bindings() {
    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn main() {
    at_bindings();
}
