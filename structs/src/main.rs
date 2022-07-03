#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
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

    assert_eq!(user1.email, "u1@example.com");

    user1.email = String::from("uu1@example.com");

    assert_eq!(user1.email, "uu1@example.com");

    let user2 = User {
        email: String::from("u2@email.com"),
        ..user1
    };

    let user3 = build_user(String::from("qwe"), String::from("asd"));

    assert_eq!(user2.email, "u2@email.com");
    assert_eq!(user3.email, "qwe");
}
