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

    scores2.insert(String::from("Red"), 20);
    scores2.insert(String::from("Green"), 50);
    scores2.insert(String::from("White"), 40);

    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    println!("length of 1: {}", scores.len());
    println!("length of 2: {}", scores2.len());

    println!("ok");
}
