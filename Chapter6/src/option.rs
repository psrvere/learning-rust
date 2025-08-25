/*
Rust doesn't have the null feature that many other languages have. In languages with null, variables can always be in one of two states
- null or not null. So if you tried to use null value as not null value, program will throw an error.

Rust acknoledges that the concept of null is a a useful one: a null is a value that is currently invalid or absent for some reason.
*/

pub fn option() {
    let some_number = Some(5); // Rust can infer type here from 5
    dbg!(some_number);

    let some_char = Some('e');
    dbg!(some_char);

    let absent_number: Option<i32> = None; // Type needs to be annotated as compiler can't infer it from value
    dbg!(absent_number);

    // Option<T> and <T> are two different types for Rust
    // For examples, we need to convert Option<u8> to <u8> before adding these two numbers
    // the "design decision" is delibrate to handle operations when one value is Null
    // We can keep a value as Option<T> when we expect it to be Null
    // But we need to explicitley convert it to <T> type and handle the scenairos where it can be Null
    // Before we do any operation with that value
    // i.e. when we have <T> type for a value, we can safely assume its value is not None
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Patterns that bind to Values

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    // only adding sample states
}

pub enum CoinUsState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents2(coin: CoinUsState) -> u8 {
    match coin {
        CoinUsState::Penny => 1, 
        CoinUsState::Nickel => 5,
        CoinUsState::Dime => 10,
        CoinUsState::Quarter(state) => {
            println!("State Quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

// Catch-all patterns

pub fn catch_all() {
    let dice_roll = 9;
    match dice_roll {
        3 => (), // using () as placeholder for a function
        7 => (),
        other => (), // will handle all other cases
    }
}

pub fn catch_all2() {
    let dice_roll = 3;
    match dice_roll {
        3 => (),
        7 => (),
        _ => (), // we don't want to use the value in catch_all pattern
    }
}