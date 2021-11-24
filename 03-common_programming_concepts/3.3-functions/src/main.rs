fn functions() {
    println!("Hello, world!");

    another_function_1();
}

fn another_function_1() {
    println!("Another function.");
}

fn function_parameters_1() {
    another_function_2(5);
}

fn another_function_2(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_parameters_2() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn function_bodies_contain_statements_and_expressions_1() {
    let y = 6;
}

fn function_bodies_contain_statements_and_expressions_2() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn functions_with_return_values_1() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn functions_with_return_values_2() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    functions_with_return_values_2();
}
