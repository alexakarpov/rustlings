#[derive(Debug, PartialEq, Copy, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Option<Coin>) -> u8 {
    match coin {
        Some(Coin::Penny) => 1,
        Some(Coin::Nickel) => 5,
        Some(Coin::Dime) => 10,
        Some(Coin::Quarter) => 25,
        None => 0
    }
}

fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let q1 = Coin::Quarter;
    let q2 = Coin::Quarter;
    println!("{}", value_in_cents(Some(p)));
    println!("{}", value_in_cents(Some(n)));
    println!("{}", value_in_cents(Some(q1)));
    // the code below won't work after the variables are moved
    // in value_in_cents calls, unless Copy and Clone are derived
    assert_ne!(Some(p), Some(n));
    assert_eq!(value_in_cents(Some(q1)), value_in_cents(Some(q2)));
}
