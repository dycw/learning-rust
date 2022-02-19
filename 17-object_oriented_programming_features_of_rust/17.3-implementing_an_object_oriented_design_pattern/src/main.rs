// Implementing an Object-Oriented Design Pattern

use implementing_an_object_oriented_design_pattern::Post;

fn implementing_an_object_oriented_design_pattern() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn main() {
    implementing_an_object_oriented_design_pattern();
}
