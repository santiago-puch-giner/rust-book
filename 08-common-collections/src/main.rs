use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
enum Food {
    Apple,
    Banana,
    Tofu,
    Tempeh,
    Lentils,
    Pasta,
    Rice,
}

fn print_vector<D: Debug>(vec: &Vec<D>) {
    for v in vec {
        println!("{:?}", v);
    }
}

fn double_vector(vec: &mut Vec<u8>) {
    for v in vec.iter_mut() {
        *v *= 2;
    }
}

fn print_utf_16(s: &str) {
    let mut ut8_byte = [0; 4];
    for c in s.chars() {
        let hex = c.encode_utf16(&mut ut8_byte);
        println!("{c} ->");
        for hex_v in hex {
            println!("{hex_v}");
        }
    }
}

fn main() {
    /***********/
    /* VECTORS */
    /***********/

    // Creating an empty vector, adding data and mutating it
    let mut vec: Vec<u8> = Vec::new();
    vec.push(10);
    let other_vec = vec![1, 2, 3, 4, 5];
    vec.extend(other_vec);
    vec.remove(1);
    println!("{:?}", vec);

    // Reading elements of a vector
    let second = &vec[1];
    println!("{}", second);

    let second_option = vec.get(0);
    match second_option {
        None => println!("No value in position 0 now"),
        Some(v) => println!("{}", v),
    }

    // Iterate over vector
    print_vector(&vec);

    // Mutate vector element by element
    double_vector(&mut vec);
    print_vector(&vec);

    // Vector of enums
    let groceries: Vec<Food> = vec![
        Food::Banana,
        Food::Tofu,
        Food::Tofu,
        Food::Rice,
        Food::Pasta,
        Food::Tempeh,
        Food::Apple,
        Food::Lentils,
    ];
    print_vector(&groceries);

    /***********/
    /* STRINGS */
    /***********/
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // Strings can't be indexed due to how Rust stores them in memory
    // let h = s[0]; --> the trait `SliceIndex<str>` is not implemented for `{integer}`
    //
    // // But one can iterate its chars
    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    print_utf_16("どういたしまして");
    print_utf_16("Hallo, guten Abend!");

    /*************/
    /* HASH MAPS */
    /*************/

    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    map.insert("Red".to_string(), 100);
    map.insert("Green".to_string(), 5);

    // Get values from a map
    let key = "Green";
    let score = map.get(key);
    if let Some(v) = score {
        println!("score for {key} -> {v}");
    }

    let mut index = HashMap::new();
    index.insert(1, String::from("Madrid"));
    index.insert(2, String::from("Barcelona"));
    index.insert(3, String::from("Berlin"));
    index.insert(4, String::from("München"));
    index.insert(5, String::from("New Jersey"));

    let city = index.get(&2);
    if let Some(known_city) = city {
        println!("{known_city}");
    }

    // Adding a key and a value only if a key is not present
    let entry = map.entry(String::from("Orange"));
    entry.and_modify(|v| *v += 1).or_insert(9); // expected 9
    map.entry(String::from("Yellow")).or_insert(50);
    map.entry(String::from("Red")).or_insert(1);

    println!("{map:?}");

    let entry = map.entry(String::from("Orange"));
    entry.and_modify(|v| *v += 1).or_insert(9); // expected 9+1=10

    println!("{map:?}");
}
