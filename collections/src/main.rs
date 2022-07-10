use std::io;

fn main() {
    let mut v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    loop {
        println!("enter [0-based] index: ");
        let mut idx = String::new();
        io::stdin()
            .read_line(&mut idx)
            .expect("failed to read input");

        let index: usize = match idx.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match v.get(index) {
            Some(thing) => println!("The element at {} is {}", index, thing),
            None => println!("There is no {} element.", index),
        }
    }
}
