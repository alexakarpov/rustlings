struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // what if we skip the ref?
    );
}

fn area(rectangle: &Rectangle) -> u32 { // what if ^
    rectangle.width * rectangle.height
}
