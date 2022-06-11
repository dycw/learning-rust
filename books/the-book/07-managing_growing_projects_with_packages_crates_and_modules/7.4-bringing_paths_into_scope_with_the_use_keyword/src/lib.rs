// Bringing Paths into Scope with the use Keyword

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;

// use self::front_of_house::hosting;

pub fn eat_at_restaurant_1() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant_2() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// Providing New Names with the as Keyword

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

// Re-exporting Names with pub use

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Using Nested Paths to Clean Up Large use Lists

use std::io;
use std::io::Write;
use std::io::{self, Write};
