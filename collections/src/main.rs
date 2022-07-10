use std::io;

fn main() {
    // let v1: Vec<i32> = Vec::new();
    // infer int32: (note the ! macro)
    let mut v = vec![1, 2, 3];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    loop {
        println!("pick index");
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
