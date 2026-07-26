// Lifetime annotations don’t change how long any of the references live.
// Rather, they describe the relationships of the lifetimes of multiple references to each other without
// affecting the lifetimes.

// Lifetimes notation

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// EXAMPLE
// The function's return type contains a borrowed value, but it does not specify whether it borrows it from x or y
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// Declare a generic lifetime and assign it accordingly
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Structs with references need to declare their lifetimes
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

pub fn create_excerpt_from_string(content: &str) -> ImportantExcerpt {
    let important_exceprt = ImportantExcerpt { part: content };
    important_exceprt
}
