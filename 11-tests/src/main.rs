fn main() {
    println!("Hello, world!");
    println!("{}", add(5, 7));
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

// Run with `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_works_2() {
        let result = add(128, 128);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_fails_on_purpose() {
        panic!("this test failes, oh no!");
    }

    #[test]
    fn test_with_a_wrong_assertion() {
        let result = add(3, 2);
        assert_eq!(
            result, 4,
            "Why is 3+2 not 4? 🤨" // custom assertion messages
        );
    }

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn test_guess_should_panic_when_value_0() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn test_guess_should_panic_when_value_lt_0() {
        Guess::new(-10);
    }

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn test_guess_should_panic_when_value_gt_100() {
        Guess::new(101);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        let result = add(10, 10);
        if result == 20 {
            Ok(())
        } else {
            Err(format!("Expected 20, got {}", result))
        }
    }
}
