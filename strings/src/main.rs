fn main() {
    let data = "initial contents";

    let s1 = data.to_string();

    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");

    assert_eq!(s1, s2);
    assert_eq!(s3, s2);

    let mut foo = String::from("foo");
    let bar = "bar";
    foo.push_str(bar); // not taking ownership of bar
    foo.push('!'); // push vs push_str
    assert_eq!(foo, "foobar!"); // huh? String and &str / slice?

    /*
        ...compiler can coerce the &String argument into a &str
        see "deref coercion"
        (as is wirh rhe '+' below)
    */
    assert_eq!(bar, "bar");

    let hello = String::from("Hello");
    let world = String::from("world");
    let s3 = hello + ", " + &world + "!"; // note s1 has been moved here and can no longer be used
    assert_eq!(s3, "Hello, world!");
    assert_eq!(world, "world"); // still ok, unlike s1
                                //assert_eq!(s1, "Hello, "); // LHS of '+' no longer owned

    // let s = format!("{}{}", hello, world);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    // ^ uses references hence takes no ownership
    let hello = String::from("你好");
    assert_eq!(s, "tic-tac-toe");
    println!("All good - {}", hello);
}
