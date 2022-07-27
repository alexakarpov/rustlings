//     Given a list of integers, use a vector and return the median
//     (when sorted, the value in the middle position) and mode (the
//     value that occurs most often; a hash map will be helpful here)
//     of the list.

//     Convert strings to pig latin. The first consonant of each word
//     is moved to the end of the word and “ay” is added, so “first”
//     becomes “irst-fay.” Words that start with a vowel have “hay”
//     added to the end instead (“apple” becomes “apple-hay”). Keep in
//     mind the details about UTF-8 encoding!

//     Using a hash map and vectors, create a text interface to allow
//     a user to add employee names to a department in a company. For
//     example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
//     let the user retrieve a list of all people in a department or all
//     people in the company by department, sorted alphabetically.

use std::collections::HashMap;

fn main() {
    let mut si = [2, 12, 1, 45, 14, 3, 16, 29, 9, 12, 45, 28, 45];
    println!("mutable array: {:?}, of {:?} elements", si, report(&si));
    let med = median(&mut si);
    let ave: f32 = average(&si);
    let mode = mode(&si);
    println!("median is {:?}", med);
    println!("average is {:?}", ave);
    println!("mode is {:?}", mode);
}

/*
fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}
*/

fn mode(numbers: &[i32]) -> i32 {
    let mut all: HashMap<i32, i32> = HashMap::new();

    for &n in numbers {
        *all.entry(n).or_insert(0) += 1;
    }

    let mut vals: Vec<(&i32, &i32)> = all.iter().collect();
    println!("vals: {:?}", vals);
    let mut max: i32 = 0;
    for (k, v) in vals {
        println!("max is {:?}, v is {:?}", max, v);
        if max < *v {
            max = *v;
        }
    }
    max
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn report(numbers: &[i32]) -> usize {
    numbers.len()
}

// fn piglatin
