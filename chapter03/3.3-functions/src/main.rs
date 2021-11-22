fn f1() {
    println!("Hello, world!");

    f2();
}

fn f2() {
    println!("Another function.");
}

fn f3() {
    f4(5);
}

fn f4(x: i32) {
    println!("The value of x is: {}", x);
}

fn f5() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn f6() {
    let y = 6;
}

fn f7() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn f8() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
