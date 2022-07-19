use std::collections::HashMap;
use std::mem;

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

    println!("=====================================");

    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", xs);

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500]; //500 zeroes

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // Arrays are stack allocated
    println!(
        "array of {} occupies {} bytes",
        xs.len(),
        mem::size_of_val(&xs)
    );

    println!(
        "'si' vector of {} elements occupies {} bytes",
        si.len(),
        mem::size_of_val(&si)
    );

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
