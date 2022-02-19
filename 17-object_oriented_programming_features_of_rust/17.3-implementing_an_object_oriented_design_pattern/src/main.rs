// Implementing an Object-Oriented Design Pattern

use implementing_an_object_oriented_design_pattern::Post4;

fn implementing_an_object_oriented_design_pattern() {
    let mut post = Post4::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

// Encoding States and Behavior as Types

use implementing_an_object_oriented_design_pattern::Post5;

fn encoding_states_and_behavior_as_types() {
    let mut post = Post5::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

fn main() {
    encoding_states_and_behavior_as_types();
}
