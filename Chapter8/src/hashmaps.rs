use std::collections::HashMap;
// HashMaps need explicit import
// String, Vec, i32, bool, char, etc. are a few examples of Prelude
// Prelude is automatically imported
// Prelude includes most commonly used types

pub fn hashmaps() {
    // Creating a new Hash Map
    let mut scores = HashMap::new();    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 12);
    // HashMaps also store data on the heap
    // HashMaps are homogeneous i.e. all the keys must have same type and
    // all the values must have same type

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    // .get() will give Option<&i32>
    let blue_score = scores.get(&team_name);

    // .copied() will copy it to make Option<i32> value
    let blue_score = scores.get(&team_name).copied();

    // .unwrap_or will give i32 value
    let blue_score = scores.get(&team_name).copied().unwrap_or(0);
    println!("blue team score: {blue_score}");

    // Iterating over a Hash Map
    // this will print key, value in arbitrary order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Updating a HashMap

    // 1. Overwriting a Value
    scores.insert(String::from("Blue"), 15);
    println!("{:?}", scores);

    // 2. Adding a key and value only if key is not present
    scores.entry(String::from("Blue")).or_insert(20); // will not update as Blue key exists
    scores.entry(String::from("Yellow")).or_insert(20); // will add Yellow key with value 20
    println!("{:?}", scores);

    // 3. Updating a value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}