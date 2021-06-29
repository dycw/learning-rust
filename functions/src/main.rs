fn main() {
    let user1 = User {
        username: String::from("derek"),
        email: String::from("d.wan@icloud.com"),
        sign_in_count: 1,
        active: true,
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", origin)
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
