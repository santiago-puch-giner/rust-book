use crate::USState::{Alabama, Alaska};

enum Message {
    Quit,                       // has no data associated with it
    Move { x: i32, y: i32 },    // has named fields, like a struct
    Write(String),              // has a type, in this case a String
    ChangeColor(i32, i32, i32), // has a tuple
}

impl Message {
    // The advantage of grouping these message types with heterogeneus data is that you can group their functionality
    // The alternative (defining a struct for each message type, like struct QuitMessage, struct MoveMessage, etc.)
    // does not facilitate that.

    fn decode(&self) {
        // Implement code here
    }

    fn encode(&self) {
        // Implement code here
    }
}

/** Option Enum */
// This is how the Option enum is defined in the stl
enum MyOption<T> {
    None,
    Some(T),
}

/* Match control flow */
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    // ... and more
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn coin_value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn main() {
    println!("Hello, world!");

    /* Enums */

    let msg = Message::Quit;
    msg.decode();
    msg.encode();
    let msg = Message::Move { x: 1, y: 4 };
    msg.encode();
    msg.decode();

    /* Option enum */
    let mut some_number: Option<i32> = None;
    println!("Some number is now: {:?}", some_number);
    some_number = Some(1);
    println!("Some number is now: {:?}", some_number);

    /* Match control flow */
    let penny = Coin::Penny;
    println!("{:?} value = {:?}", penny, coin_value_in_cents(&penny));

    let dime = Coin::Dime;
    println!("{:?} value = {:?}", dime, coin_value_in_cents(&dime));

    let quarter = Coin::Quarter(USState::Alaska);
    println!("{:?} value = {:?}", quarter, coin_value_in_cents(&quarter));

    match some_number {
        None => println!("We don't have a number"),
        Some(v) => println!("The value of the number is {:?}", v),
    }

    // Matches are exhaustive
    let dice_roll = 9;
    match dice_roll {
        1 => println!("You loose the fight"),
        3 => println!("Your enemy is put to sleep, they skip their next turn"),
        7 => println!("You win the battle, +10xp"),
        num => println!("The rolled number has no effect ({num})"),
    }

    // One can use the _ since we don't need the catch-all value, and the unit tuple () to do nothing
    match dice_roll {
        1 => (),
        _ => (),
    }

    // Consider move vs reference semantics with match
    let opt: Option<String> = Some(String::from("Hello!"));

    match &opt {
        Some(s) => println!("Some {s}!"),
        None => println!("None!"),
    }
    println!("{opt:?}"); // this wouldn't work if we moved the opt into s and then dropped it (no references)

    /* `if let` & `let...else` constructs */

    // Instead of writing this
    let config_max: Option<u8> = Some(3);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // One can do
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }

    // And if we need to catch the "else" case, we can extend the construct with it
    let mut count = 0;
    let coins = [
        Coin::Quarter(Alabama),
        Coin::Dime,
        Coin::Dime,
        Coin::Nickel,
        Coin::Quarter(Alaska),
        Coin::Penny,
    ];
    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {state:?}")
        } else {
            count += 1;
        }
    }
    println!("Count: {count}");

    // We can also use the let..else construct
    for coin in coins.iter() {
        let Coin::Quarter(state) = coin else {
            continue;
        };
        match state {
            USState::Alabama => println!("Catched a quarter from Alabama!"),
            USState::Alaska => println!("Catched a quarter from Alaska!"),
        }
    }
}
