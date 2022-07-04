#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // what if ^
        self.width * self.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // what if we skip the ref? which is really an immutable borrow..
    );

    dbg!(&rect1);
    println!("rect1 is {:#?}", rect1);
}
