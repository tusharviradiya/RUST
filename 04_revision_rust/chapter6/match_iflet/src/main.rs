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
    let a = Coin::Quarter;
    let b = Coin::Penny;
    let c = Coin::Dime;
    let d = Coin::Nickel;
    println!("{}", value_in_cents(a));
    println!("{}", value_in_cents(b));
    println!("{}", value_in_cents(c));
    println!("{}", value_in_cents(d));
}