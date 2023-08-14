use std::collections::HashMap;

fn main() {
    // COMMON COLLECTIONS IN RUST

    // Vectors

    let numbers = vec![1, 2, 3, 4];

    let mut names: Vec<String> = Vec::new();

    names.push("Alice".to_string());
    names.push("Bob".to_string());

    let first_name = &names[0];
    let second_name = &names[1];

    println!("Name-1: {first_name}\nName-2: {second_name}");

    names.pop(); // Bob popped

    println!("{:?}", names);

    //iterating
    for number in &numbers {
        println!("number: {number}")
    }

    let slice = &numbers[1..3];

    println!("{:?}", slice);

    // Strings

    let str = String::from("hello");

    //iterating via chars
    for c in str.chars() {
        println!("{c}")
    }

    //iterating via bytes
    for b in str.as_bytes() {
        println!("{b}")
    }

    // Hashmaps

    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 10); // [Alice: 10]
    scores.insert(String::from("Bob"), 5); // [Alice: 10, Bob: 5]

    let first_score = scores.get(&String::from("Alice"));

    println!("{:?}", first_score); //not the value itself but "Some(10)"

    println!("{:?}", scores);

    scores.remove(&String::from("Bob"));

    println!("{:?}", scores);

    //iterating
    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
}
