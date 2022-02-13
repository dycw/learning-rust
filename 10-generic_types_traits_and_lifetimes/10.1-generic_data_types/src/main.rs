// In Function Definitions

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn in_function_definitions_1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn in_function_definitions_2() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// In Struct Definitions

struct Point1<T> {
    x: T,
    y: T,
}

fn in_struct_definitions_1() {
    let integer = Point1 { x: 5, y: 10 };
    let float = Point1 { x: 1.0, y: 4.0 };
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn in_struct_definitions_2() {
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_floatf = Point2 { x: 5, y: 4.0 };
}

// In Enum Definitions

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// In Method Definitions

impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn in_method_definitions_1() {
    let p = Point1 { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn in_method_definitions_2() {
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Performance of Code Using Generics

fn performance_of_code_using_generics_1() {
    let integer = Some(5);
    let float = Some(5.0);
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn performance_of_code_using_generics_2() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

fn main() {
    performance_of_code_using_generics_2();
}
