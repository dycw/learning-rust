// Creating a Reference Cycle

use crate::List::{Cons, Nil};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn creating_a_reference_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

// Creating a Tree Data Structure: a Node with Child Nodes

#[derive(Debug)]
struct Node1 {
    value: i32,
    children: RefCell<Vec<Rc<Node1>>>,
}

fn creating_a_tree_data_structure_a_node_with_child_nodes() {
    let leaf = Rc::new(Node1 {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node1 {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}

// Adding a Reference from a Child to Its Parent

use std::rc::Weak;

#[derive(Debug)]
struct Node2 {
    value: i32,
    parent: RefCell<Weak<Node2>>,
    children: RefCell<Vec<Rc<Node2>>>,
}

fn adding_a_reference_from_a_child_to_its_parent() {
    let leaf = Rc::new(Node2 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node2 {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// Visualizing Changes to strong_count and weak_count

fn visualizing_changes_to_strong_count_and_weak_count() {
    let leaf = Rc::new(Node2 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node2 {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
fn main() {
    visualizing_changes_to_strong_count_and_weak_count();
}
