//! Crate to add one

use rand;

use std::ops::Add;

///Adds one to the provided value
///
/// Accepts any type that implements the `Add` trait with an u32 type
///
/// # Examples
///
/// ```
/// let num: u32 = 5;
/// let answer = add_one::add_one(num);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one<T: Add<u32, Output = T>>(x: T) -> T {
    x + 1
}

pub fn add_rand<T: Add<u32, Output = T>>(x: T) -> T {
    let rand_num: u32 = rand::random();
    x + rand_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(3);
        assert_eq!(result, 4);
    }
}
