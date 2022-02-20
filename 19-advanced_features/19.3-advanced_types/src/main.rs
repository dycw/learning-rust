// Creating Type Synonyms with Type Aliases

type Kilometers = i32;

fn creating_type_synonyms_with_type_aliases_1() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

fn creating_type_synonyms_with_type_aliases_2() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        unimplemented!()
    }
}

fn creating_type_synonyms_with_type_aliases_3() {
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        unimplemented!()
    }
}

use std::fmt;
use std::io::Error;

pub trait Write1 {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

type Result2<T> = std::result::Result<T, std::io::Error>;

pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> Result2<usize>;
    fn flush(&mut self) -> Result2<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result2<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
}

// The Never Type that Never Returns

fn bar() -> ! {
    unimplemented!()
}

fn the_never_type_that_never_returns() -> ! {
    print!("forever ");

    loop {
        print!("and ever ");
    }
}

// Dynamically Sized Types and the Sized Trait

fn dynamically_sized_types_and_the_sized_trait() {
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}

fn generic_1<T>(t: T) {
    // --snip--
}

fn generic_2<T: Sized>(t: T) {
    // --snip--
}

fn generic_3<T: ?Sized>(t: &T) {
    // --snip--
}

fn main() {
    the_never_type_that_never_returns();
}
