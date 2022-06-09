// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("exit");
    }

    println!("Excercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Excercise Failed if printing out this line!");
}
