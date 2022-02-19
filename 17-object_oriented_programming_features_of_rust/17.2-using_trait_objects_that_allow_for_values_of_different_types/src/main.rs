// Implementing the Trait

use using_trait_objects_that_allow_for_values_of_different_types::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
use using_trait_objects_that_allow_for_values_of_different_types::{Button, Screen1};

fn implementing_the_trait() {
    let screen = Screen1 {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

fn main() {
    implementing_the_trait();
}
