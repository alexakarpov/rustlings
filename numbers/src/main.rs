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
    println!("x is {x}, y is {y}, z is {z} and last is a unit");

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

    println!("please enter an array index (0 to 11):");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    println!("entered {index} (a string with a newline)");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = months[index];

    println!("The value of the element at index {index} is: {element}");
}

// can be declared in a function scope, and after the invocation
fn _inc(n: i32) -> i32 {
    n + 1
}
