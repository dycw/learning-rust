// The match Control Flow Operator

enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents_1(coin: Coin1) -> u8 {
    match coin {
        Coin1::Penny => 1,
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter => 25,
    }
}

fn value_in_cents_2(coin: Coin1) -> u8 {
    match coin {
        Coin1::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter => 25,
    }
}

// Patterns that Bind to Values

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_3(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>

fn matching_with_option_T() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// Catch-all Patterns and the _ Placeholder

fn catch_all_patterns_and_the___placeholder_1() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

fn catch_all_patterns_and_the___placeholder_2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

fn catch_all_patterns_and_the___placeholder_2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

fn main() {
    println!("Hello, world!");
}
