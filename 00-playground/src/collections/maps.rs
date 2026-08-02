use std::{collections::HashMap, fmt::Display};

pub fn fun_with_hash_maps() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 100);

    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("{score}");

    // We can iterate over a hash map similarly to how we iterate over a vector
    for (k, v) in &scores {
        println!("{k} -> {v}");
    }

    // By default, if we insert with an existing key, it overwrites the value
    scores.insert("Blue".to_string(), 5);
    println!("After inserting Blue for a second time");
    print_map(&scores);

    // We can also insert only it it does not exist with the following recipe
    scores.entry(String::from("Blue")).or_insert(1000);
    scores.entry(String::from("Red")).or_insert(1000);
    println!("After optionally inserting Blue and Red");
    print_map(&scores);
}

fn print_map<K: Display, V: Display>(map: &HashMap<K, V>) {
    for (k, v) in map {
        println!("{k} -> {v}");
    }
}
