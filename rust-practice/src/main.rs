pub mod numerical;

pub use crate::numerical::recursive::fibonacci;

fn main() {
    println!("Hello, world!");

    println!("{}", fibonacci(0));
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(5));
    println!("{}", fibonacci(6));
}
