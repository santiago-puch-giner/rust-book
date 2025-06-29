pub mod collections;
pub mod numerical;

use crate::collections::strings;
use crate::collections::vectors;
use crate::numerical::recursive;

fn run_fibonacci() {
    println!("Hello, world!");

    println!("{}", recursive::fibonacci(0));
    println!("{}", recursive::fibonacci(1));
    println!("{}", recursive::fibonacci(2));
    println!("{}", recursive::fibonacci(3));
    println!("{}", recursive::fibonacci(4));
    println!("{}", recursive::fibonacci(5));
    println!("{}", recursive::fibonacci(6));
}

fn main() {
    println!("Running fibonacci series...");
    run_fibonacci();

    println!("Fun with collections: vectors");
    vectors::play_with_vectors();

    println!("Fun with collections: strings");
    strings::fun_with_strings();

    println!("End of main")
}
