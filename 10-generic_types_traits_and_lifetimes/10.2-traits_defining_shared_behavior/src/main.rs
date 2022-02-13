// Defining a Trait

pub trait Summary1 {
    fn summarize_1(&self) -> String;
}

// Implementing a Trait on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary1 for NewsArticle {
    fn summarize_1(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary1 for Tweet {
    fn summarize_1(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn implementing_a_trait_on_a_type_1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize_1());
}

// Default Implementations

pub trait Summary2 {
    fn summarize_2(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary2 for NewsArticle {}

fn implementing_a_trait_on_a_type_2() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize_2())
}

pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn implementing_a_trait_on_a_type_3() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// Traits as Parameters

pub fn notify_1(item: &impl Summary1) {
    println!("Breaking news! {}", item.summarize_1());
}

// Trait Bound Syntax

pub fn notify_2<T: Summary1>(item: &T) {
    println!("Breaking news! {}", item.summarize_1());
}

pub fn notify_3(item1: &impl Summary1, item2: &impl Summary1) {}

pub fn notify_4<T: Summary1>(item1: &T, item2: &T) {}

// Specifying Multiple Trait Bounds with the + Syntax

use std::fmt::Display;

pub fn notify_5(item: &(impl Summary1 + Display)) {}

pub fn notify_6<T: Summary1 + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses

use std::fmt::Debug;

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// Returning Types that Implement Traits

fn returns_summarizable_1() -> impl Summary1 {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// fn returns_summarizable_2(switch: bool) -> impl Summary1 {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Fixing the largest Function with Trait Bounds

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn fixing_the_largest_function_with_trait_bounds() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Using Trait Bounds to Conditionally Implement Methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    fixing_the_largest_function_with_trait_bounds();
}
