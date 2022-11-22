enum IpAddrType {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32}, // <- named fields like a struct
    Write (String), // <- single string
    ChangeColor (i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --skip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8{
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn remove_fancy_hat(){}
fn add_fancy_hat(){}
fn move_player(moves: i32){}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let first_coin = Coin::Nickel;
    println!("The value of the first coin is: {}", value_in_cents(&first_coin));

    let second_coin = Coin::Penny;
    println!("The value of the second coin is: {}", value_in_cents(&second_coin));

    let third_coin = Coin::Quarter(UsState::Alabama);
    println!("The value of the third coin is: {}", value_in_cents(&third_coin));

    // println!("Result of none + 1: {}", plus_one(None));
    // println!("Result of Some(5) + 1: {}", plus_one(Some(5)));

    let dice_roll = 9;

    match dice_roll {
        7 => remove_fancy_hat(),
        3 => add_fancy_hat(),
        _ => (),
    }

    match dice_roll {
        7 => remove_fancy_hat(),
        3 => add_fancy_hat(),
        other => move_player(other),
    }

    let config_max = Some(3u8);

    if let Some(max) = config_max {  // Alternativ to match with just one used arm 
        println!("The maximum configured is: {}", max);
    }

}


