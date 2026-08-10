use add_one::{add_one, add_rand};

fn main() {
    let num: u32 = 1;
    let rand_num: u32 = rand::random();
    println!(
        "Hello, world! {rand_num} plus one is {}!",
        add_one(rand_num),
    );
    println!(
        "Hello, world! {num} plus a random number is {}!",
        add_rand(num)
    );
}
