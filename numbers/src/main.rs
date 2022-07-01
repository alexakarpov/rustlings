/*
Wrap in all modes with the wrapping_* methods, such as wrapping_add
Return the None value if there is overflow with the checked_* methods
Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
Saturate at the value’s minimum or maximum values with saturating_* methods

*/

use std::io;

fn main() {
    let u = println!("Hello world");
    let tup = (12, 42.2, 33 / 2, u);
    let (x, y, z, _) = tup;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // error[E0277]: `()` doesn't implement `std::fmt::Display`
    println!("x is {x}, y is {y}, z is {z} and last is a {{u}}");

    another_function();

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = months[index];

    println!("The value of the element at index {index} is: {element}");
}

// can be declared in a function scope, and after the invocation
fn another_function() {
    println!("another f")
}
