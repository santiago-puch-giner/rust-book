pub fn fun_with_strings() {
    // Let's create a new string, interpeted as a collection of bytes encoded in UTF8
    let mut s = String::new();

    let data = "This is a string literal that converts to a string".to_string();
    s.push_str(&data);
    s.push_str(", and I could actually append the string literal directly.");

    println!("{s}");

    let s2 = String::from("This is another common way to create strings in Rust");
    println!("{s2}");

    // We can also push single characters
    s.push(' ');
    s.push('C');
    s.push('h');
    s.push('a');
    s.push('r');
    s.push('s');
    s.push('!');
    println!("{s}");

    // We can concatenate multiple strings with the + operator.
    // Consider that it takes ownership of the first string.
    let prefix: String = "Prefix".to_string();
    let root: String = "Root".to_string();
    let suffix: String = "Suffix".to_string();
    let complete: String = prefix + "-" + &root + "-" + &suffix;
    // At this point, 'prefix' does not exist anymore, since 'complete' has taken ownership of it
    // println!("{prefix}"); --> this does not compile, trying to borrow from a moved value
    println!("{complete}");

    // An easier way to format would be
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s: String = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // Finally, on the topic of indexing: it is simply not so easy, string are complex, so this statement does not
    // compile
    // let answer = &s[0];

    // One can use iterators over chars or bytes to get that information
    let devangari_word = "नमस्ते".to_string();
    for c in devangari_word.chars() {
        println!("{c}");
    }
}
