use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();    
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team = "Blue".to_string();
    // get returns Option<&i32> .copied() returns Option<i32>, 
    // and .unwrap_or(default) returns x in Some(x) or default
    println!("Team {team} has a score of {}", scores.get(&team).copied().unwrap_or(0));
    
    // looping through hashmaps
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    // Handling duplicating keys
    // 1. Overwriting
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 24);

    println!("{scores:?}");

    // 2. Inserting only if no key is present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    // Gets the Entry corrosponding to the key. If it is does not exist, inserts 50.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
    
    // 3. Updating values
    let text = "what a world wonderful world";

    let mut map = HashMap::new();

    // Counts each instance of a word in the string.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{map:?}");
}
