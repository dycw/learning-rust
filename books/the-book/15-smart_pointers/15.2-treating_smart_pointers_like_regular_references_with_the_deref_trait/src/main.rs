// Following the Pointer to the Value with the Dereference Operator

fn following_the_pointer_to_the_value_with_the_dereference_operator() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Using Box<T> Like a Reference

fn using_box_t_like_a_reference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining Our Own Smart Pointer

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn defining_our_own_smart_pointer() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Treating a Type Like a Reference by Implementing the Deref Trait

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implicit Deref Coercions with Functions and Methods

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn implicit_deref_coercions_with_functions_and_methods_1() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn implicit_deref_coercions_with_functions_and_methods_2() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

fn main() {
    implicit_deref_coercions_with_functions_and_methods_2();
}
