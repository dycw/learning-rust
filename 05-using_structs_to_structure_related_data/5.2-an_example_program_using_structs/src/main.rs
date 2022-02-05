fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

fn an_example_program_using_structs() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_1(width1, height1)
    );
}

// Refactoring with Tuples

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn refactoring_with_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );
}

// Refactoring with Structs: Adding More Meaning

struct Rectangle1 {
    width: u32,
    height: u32,
}

fn area_3(rectangle: &Rectangle1) -> u32 {
    rectangle.width * rectangle.height
}

fn refactoring_with_structs_adding_more_meaning() {
    let rect1 = Rectangle1 {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_3(&rect1)
    )
}

// Adding Useful Functionality with Derived Traits

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

fn adding_useful_functionality_with_derived_traits_1() {
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

fn adding_useful_functionality_with_derived_traits_2() {
    let scale = 2;
    let rect1 = Rectangle2 {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn main() {
    adding_useful_functionality_with_derived_traits_2();
}
