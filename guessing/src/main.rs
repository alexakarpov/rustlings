use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you've guessed: {guess}");

    let x = 2;
    let y = 3;
    println!("x is {x}, y is {y}")
}
