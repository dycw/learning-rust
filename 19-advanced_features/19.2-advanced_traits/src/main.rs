// Default Generic Type Parameters and Operator Overloading

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point1 {
    x: i32,
    y: i32,
}

impl Add for Point1 {
    type Output = Point1;

    fn add(self, other: Self) -> Point1 {
        Point1 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_generic_type_parameters_and_operator_overloading() {
    assert_eq!(
        Point1 { x: 1, y: 0 } + Point1 { x: 2, y: 3 },
        Point1 { x: 3, y: 3 }
    )
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn fully_qualified_syntax_for_disambiguation_calling_methods_with_the_same_name_1() {
    let person = Human;
    person.fly();
}

fn fully_qualified_syntax_for_disambiguation_calling_methods_with_the_same_name_2() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn fully_qualified_syntax_for_disambiguation_calling_methods_with_the_same_name_3() {
    println!("A baby dog is called a {}", Dog::baby_name());
}

fn fully_qualified_syntax_for_disambiguation_calling_methods_with_the_same_name_4() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// Using Supertraits to Require One Trait’s Functionality Within Another Trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Using the Newtype Pattern to Implement External Traits on External Types

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn using_the_newtype_pattern_to_implement_external_traits_on_external_types() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

fn main() {
    using_the_newtype_pattern_to_implement_external_traits_on_external_types();
}
