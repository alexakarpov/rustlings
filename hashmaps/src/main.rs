use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let teams_copy = teams.clone();
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    for t in &teams_copy {
        let s = scores.get(t);
        let s = match s {
            None => 0,
            Some(x) => *x,
        };

        println!("team {} score: {}", t, s);
    }

    let mut scores2 = HashMap::new();

    // these calls will borrow the map as mutable, so mut is required ^
    scores2.insert(String::from("Red"), 20);
    scores2.insert(String::from("Green"), 50);
    scores2.insert(String::from("White"), 40);
    scores2.entry(String::from("Red")).or_insert(100); //ignored
    scores2.entry(String::from("Black")).or_insert(100);

    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    println!("length of 1: {}", scores.len());
    println!("length of 2: {}", scores2.len());

    println!("{:?}", scores2);
    let text = "hello world  wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    println!("ok");
}
