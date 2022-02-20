// Defining and Instantiating Structs

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn defining_and_instantiating_structs_1() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

fn defining_and_instantiating_structs_2() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

fn defining_and_instantiating_structs_3(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Using the Field Init Shorthand when Variables and Fields Have the Same Name

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Creating Instances From Other Instances With Struct Update Syntax

fn creating_instances_from_other_instances_with_struct_update_syntax_1() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

fn creating_instances_from_other_instances_with_struct_update_syntax_2() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        ..user1
    };
}

// Using Tuple Structs without Named Fields to Create Different Types

fn using_tuple_structs_without_named_fields_to_create_different_types() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit-Like Structs Without Any Fields

fn unit_like_structs_without_any_fields() {
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

fn main() {
    println!("Hello, world!");
}
