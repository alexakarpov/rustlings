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
    println!("ok");
}
