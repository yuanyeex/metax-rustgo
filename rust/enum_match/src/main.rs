#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{:?} is {} cents", Coin::Penny, value_in_cents(Coin::Penny));
    println!("{:?} is {} cents", Coin::Nickel, value_in_cents(Coin::Nickel));
    println!("{:?} is {} cents", Coin::Dime, value_in_cents(Coin::Dime));
    println!("{:?} is {} cents", Coin::Quarter, value_in_cents(Coin::Quarter));
}
