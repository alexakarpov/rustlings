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
    // ^ uses references hence takes no ownership:
    assert_eq!(s, "tic-tac-toe");
    assert_eq!(s1, "tic");
    assert_eq!(s2, "tac");
    assert_eq!(s3, "toe");
    // devangari
    let _dev_bytes = [
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ];
    let dev_chars = ['न', 'म', 'स', '्', 'त', 'े'];
    let dev_graphemes = ["न", "म", "स्", "ते"];

    let hello = "Здравствуйте";

    let s = &hello[0..6]; // 4 bytes make 2 characters, 0..3 => panic

    println!("dev_chars");
    for c in dev_chars {
        println!("{:?}", c);
    }
    println!("dev_graphemes");
    for g in dev_graphemes {
        println!("{:?}", g);
    }

    println!("ru bytes");
    for c in hello.bytes() {
        print!("/{}", c);
    }
    println!(); // NL
    println!("{}", hello);

    /*
    note: the only appropriate formatting traits are:
           - ``, which uses the `Display` trait
           - `?`, which uses the `Debug` trait
           - `e`, which uses the `LowerExp` trait
           - `E`, which uses the `UpperExp` trait
           - `o`, which uses the `Octal` trait
           - `p`, which uses the `Pointer` trait
           - `b`, which uses the `Binary` trait
           - `x`, which uses the `LowerHex` trait
           - `X`, which uses the `UpperHex` trait

    */
    println!("All good");
}
