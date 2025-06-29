pub mod collections;
pub mod numerical;

pub use crate::collections::vectors;
pub use crate::numerical::fibonacci;

fn run_fibonacci() {
    println!("Hello, world!");

    println!("{}", fibonacci(0));
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(5));
    println!("{}", fibonacci(6));
}

fn main() {
    println!("Running fibonacci series...");
    run_fibonacci();

    println!("Fun with collections: vectors");
    vectors::play_with_vectors();

    println!("End of main")
}
