#[derive(Debug, PartialEq, Copy, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug, Copy, Clone, PartialEq)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Massachusets,
    Nevada,
    California,
    Texas,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // to show we can
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let q1 = Coin::Quarter(UsState::Texas);
    let q2 = Coin::Quarter(UsState::Nevada);
    println!("{}", value_in_cents(p));
    println!("{}", value_in_cents(n));
    println!("{}", value_in_cents(q1));
    // the code below won't work after the variables are moved
    // in value_in_cents calls, unless Copy and Clone are derived
    assert_ne!(p, n);
    assert_eq!(value_in_cents(q1), value_in_cents(q2));
}
