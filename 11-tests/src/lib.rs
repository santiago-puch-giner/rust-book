fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(value: u64) -> u64 {
    add(value, 2)
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

// Specifies that the module (tests in this case) shouldn’t be included in the compiled result, since it is
// part of the source code
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
    #[ignore = "just to show how to ignore a test"]
    fn test_fails_on_purpose() {
        panic!("this test failes, oh no!");
    }

    #[test]
    #[ignore = "this would faild so I am ignoring it"]
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
