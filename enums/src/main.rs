#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("calling...");
        dbg!(self);
    }
}

fn main() {
    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::Quit;
    let m3 = Message::Move {x:11,y:22};
    let m4 = Message::ChangeColor(21,32,43);
    m1.call();
    m2.call();
    m3.call();
    m4.call();
}
