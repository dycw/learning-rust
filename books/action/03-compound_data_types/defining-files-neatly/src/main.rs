#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        // As `File::new()` is a completely normal function--rather than something blessed by the language--we need to tell Rust that it will be returning a `File` from this function
        File {
            name: String::from(name), // `File::new()` does little more than encapsulate the object creation syntax
            data: Vec::new(),
        }
    }

    // fn len(&self) -> usize {  //  `File::len()` takes an implicit argument `self`. You'll notice that there is no explicit argument provided on line 26.
    //   self.data.len() //  `usize` is the type returned by `Vec<T>::len()`, which is sent directly through to the caller
    // }
}

fn main() {
    let f3 = File::new("f3.txt");

    let f3_name = &f3.name; // Fields are private by default, but can be accessed within the module that defines the struct. The module system is discussed further on in the chapter.

    //let f3_length = f3.len();
    let f3_length = f3.data.len();

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3_name, f3_length);
}
