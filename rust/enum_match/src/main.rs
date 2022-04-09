use std::ptr::NonNull;

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip -- 
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}", state);
            25
        },
    }
}

fn sum_one_opt(x: Option<i32>) -> Option<i32> {
    if (x.is_none()) {
        return None;
    } else {
        let one: i32 = 1;
        return Some(x.unwrap() + one);
    }
}

fn plus_one_enum_match(x: Option<i32>) -> Option<i32> {
    // enum-match必须穷举所有的枚举值； matches in rust are exhaustive
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{:?} is {} cents", Coin::Penny, value_in_cents(Coin::Penny));
    println!("{:?} is {} cents", Coin::Nickel, value_in_cents(Coin::Nickel));
    println!("{:?} is {} cents", Coin::Dime, value_in_cents(Coin::Dime));
    println!("{:?} is {} cents", Coin::Quarter(UsState::Alabama), value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{:?} is {} cents", Coin::Quarter(UsState::Alaska), value_in_cents(Coin::Quarter(UsState::Alaska)));
    let some32 = Some(32);
    println!("some one for option 32 is {:?}", sum_one_opt(some32));
    println!("sum one with enum match for option 32 is {:?}", plus_one_enum_match(some32));
}
