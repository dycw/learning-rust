#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This is a quarter with {:?}", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);

    let v = value_in_cents(coin);

    let some_u8 = Some(0u8);

    if let Some(0) = some_u8 {
        println!("threeee");
    }
}
