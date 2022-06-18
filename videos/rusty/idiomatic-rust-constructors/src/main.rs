use idiomatic_rust_constructors::{Post, Role, User};

fn main() {
    let user1 = User::new("testuser1234".to_string()).unwrap_or_default();

    println!("{:?}", user1);

    let post1 = Post::default();

    println!("{:?}", post1);

    let post2 = Post::new("example content".to_owned());

    println!("{:?}", post2)
}
