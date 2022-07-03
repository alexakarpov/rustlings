fn main() {
    let mut s = String::from("hello1");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("s is {s}");

    let s1 = String::from("what?");
    let s2 = s1;

    println!("{} huh?", s2);

    let s = String::from("hello2"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    let (s, _len) = calculate_length(s2);

    let len = calculate_length2(&s);
    println!("The length of '{}' is {}.", s, len);

    // --------------------

    let f = first_word("qwe asd zxc");
    println!("{}", f);
    let sg = String::from("asd fgh jkl");
    let g = first_word(&sg);
    println!("{}", g);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("in take_ownership > {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("in make_copy > {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    let l = s.len(); // len() returns the length of a String
    l
}
