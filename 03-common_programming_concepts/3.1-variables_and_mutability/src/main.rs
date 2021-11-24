fn variables_and_mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn shadowing_1() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn shadowing_2() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);
}

fn main() {
    shadowing_2();
}
