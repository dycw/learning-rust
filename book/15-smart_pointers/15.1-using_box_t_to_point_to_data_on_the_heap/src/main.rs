// Using a Box<T> to Store Data on the Heap

fn using_a_box_t_to_point_to_data_on_the_heap() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// More Information About the Cons List

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn more_information_about_the_cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn main() {
    more_information_about_the_cons_list();
}
