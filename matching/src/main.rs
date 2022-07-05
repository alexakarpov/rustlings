#[derive(Debug, PartialEq, Copy, Clone)]
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
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let q1 = Coin::Quarter;
    let q2 = Coin::Quarter;
    println!("{}", value_in_cents(p));
    println!("{}", value_in_cents(n));
    println!("{}", value_in_cents(q1));
    // the code below won't work after the variables are moved
    // in value_in_cents calls, unless Copy and Clone are derived
    assert_ne!(p,n);
    assert_eq!(value_in_cents(q1), value_in_cents(q2));
}
