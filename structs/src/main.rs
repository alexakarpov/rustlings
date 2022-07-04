#![allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn xemail(u: User) -> String {
    u.email
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new(c: u32) -> Counter {
        Counter { count: c }
    }
    fn inc(&mut self) -> () {
        self.count += 1
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("u1@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    assert_eq!(String::from("qwe"), "qwe"); // huh?

    println!("{}", user1.email);

    user1.email = String::from("uu1@example.com");

    assert_eq!(user1.email, "uu1@example.com");

    let user2 = User {
        email: String::from("u2@email.com"),
        ..user1
    };

    let user3 = build_user(String::from("qwe"), String::from("asd"));

    assert_eq!(user2.email, "u2@email.com");
    assert_eq!(user3.email, "qwe");

    let (a, b) = (3, 4);
    assert_ne!(a, b);

    println!("{}", xemail(user2));

    // -----------------------
    let mut cnt = Counter::new(11);
    println!("initially counter is: {:?}", cnt);
    cnt.inc();
    println!("finally, counter is: {:?}", cnt);
    println!("once more, counter is: {:?}", cnt);
}
