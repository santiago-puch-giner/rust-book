// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// A slice is a kind of reference, so it is a non-owning pointer.

fn count_string_chars(s: String) {
    println!("'{}' has {} chars (String)", s, s.len());
}

fn count_string_slice_chars(s: &str) {
    println!("'{}' has {} char (&str)", s, s.len());
}

fn main() {
    // Create some String and some slices from it
    let my_string = String::from("test string");
    let string_slice = &my_string;
    let string_slice_1 = &my_string[0..4];
    let string_slice_2 = &my_string[5..];

    println!("{:?}", my_string);
    println!("{:?}", string_slice);
    println!("{:?}", string_slice_1);
    println!("{:?}", string_slice_2);

    // Other ways to create String and some slices from it
    let mut second_string = "hello world".to_string();
    second_string.push_str(", Santi!");
    let second_string_view = &second_string[0..5];
    let second_string_view_2 = &second_string[6..];

    println!("{:?}", second_string);
    println!("{:?}", second_string_view);
    println!("{:?}", second_string_view_2);

    // Demonstration of rules for string types in function signatures
    // count_string_chars("this does not work"); -> expected String, found &str (&'static str, to be precise)
    count_string_chars("this is a String from a static string slice".to_string());

    count_string_slice_chars("this is a static string slice");
    count_string_slice_chars(&String::from("this is a String"));
    let long_string = "This is a long string from which I will take a slice".to_owned();
    let substring = &long_string[0..10];
    count_string_slice_chars(substring);
}
