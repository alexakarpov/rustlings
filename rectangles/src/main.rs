// #![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // borrows self immutably
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: dbg!(scale * 60),
        height: 45,
    };

    println!("rect1 is {:#?}", rect1);
    println!("rect2 is {:#?}", rect2);
    println!("rect3 is {:#?}", rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // what if we skip the ref? which is really an immutable borrow..
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area() // what if we skip the ref? which is really an immutable borrow..
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area() // what if we skip the ref? which is really an immutable borrow..
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
