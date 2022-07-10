// use std::fmt;
use std::io;
// use std::string::String;

#[derive(Debug)]
enum X {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![X::Int(42), X::Float(3.14), X::Text(String::from("lol"))];

    for it in &mut v {
        match it {
            X::Int(n) => *n *= 100,
            X::Float(n) => *n *= 1000.0,
            X::Text(s) => s.make_ascii_uppercase(),
        }
    }

    for it in &v {
        dbg!(it);
    }

    // let third: &i32 = &v[2]; // unsafe reading from a vector
    // println!("The third element is {}", third);

    loop {
        println!("enter [0-based] index: ");
        let mut idx = String::new();
        io::stdin()
            .read_line(&mut idx)
            .expect("failed to read input");

        let index: usize = match idx.trim().parse() {
            Ok(999) => break, // magic exit value :)
            Ok(num) => num,
            Err(_) => continue,
        };

        match v.get(index) {
            Some(thing) => println!("The element at {} is {:?}", index, thing),
            None => println!("There is no {} element.", index),
        }
    }
}
