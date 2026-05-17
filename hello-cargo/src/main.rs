fn main() {
    // Some notes taken from "Rust for the impatient" video
    // https://www.youtube.com/watch?v=br3GIIQeefY
    // Original blogpost
    // https://fasterthanli.me/articles/a-half-hour-to-learn-rust

    /* Basic variable declaration and primitive types */
    let pi: f32 = 3.14;
    // pi = 666.666; // does not compile, all variables are immutable by default
    assert!(pi * 2.0 == 6.28);

    let mut mutable_integer: i64 = 2;
    assert!(mutable_integer == 2);
    mutable_integer = 3;
    assert!(mutable_integer == 3);

    /* Tuples */
    let tup: (&'static str, f64, bool, i32) = ("a", 0.1, true, -2);
    assert!(tup.0 == "a");
    assert!(tup.1 == 0.1);
    assert!(tup.2);
    assert!(tup.3 == -2);

    /* Vectors */
    let mut vec: Vec<f64> = Vec::new();
    vec.push(0.1);
    vec.push(0.2);
    assert!(vec.len() == 2);

    let vec_int: Vec<u8> = vec![0, 1, 0, 1, 1, 0, 128]; // macro(!) to create a vector
    assert!(vec_int[0] == 0);

    /* Functions */
    fn greet() {
        println!("Hello person");
    }
    greet();

    fn dice_roll() -> i32 {
        4 // no semicolon indicates that this is the *tail*, equivalent to `return 4;`
    }
    assert!(dice_roll() == 4);

    /* Blocks (like in C / C++) */
    let x = "outside the scope of a block";
    {
        let x = "inside the scope of a block";
        println!("{}", x);
    }
    println!("{}", x);
    // Blocks are also expressions, meaning that this is valid:
    let linear_eq: f64 = {
        let x: f64 = 0.5; // statement 1
        let a: f64 = 2.0; // statement 2
        let b: f64 = -5.0; // statement 3
        a * x + b // *tail* statement, equivalent to `return a * x + b;`
    };
    assert!(linear_eq == -4.0);

    /* Namespaces ~ crate::file::function */
    let min_value: i32 = std::cmp::min(1, -1);
    assert!(min_value == -1);

    use std::cmp::max;
    let max_value: i32 = max(1, -1);
    assert!(max_value == 1);

    assert!(str::ends_with("file.zip", ".zip")); // types (like `str`) are also namespaces

    /* Structs */
    struct Number {
        odd: bool,
        value: i64,
    }
    impl Number {
        fn is_positive(self) -> bool {
            self.value > 0
        }
    }
    let x = Number {
        odd: false,
        value: 16,
    };
    assert!(!x.odd);
    assert!(x.value == 16);
    assert!(x.is_positive());

    /* Generics */
    fn generic_addition<T>(_v1: T, _v2: T) {
        // Work with those arguments
    }
    generic_addition(1, 2); // does nothing but it's cool anyway

    struct Pair<T> {
        lh: T,
        rh: T,
    }
    let mut p: Pair<f64> = Pair { lh: 1.5, rh: 2.5 };
    p.lh = p.rh;
    p.rh = p.lh;
    assert!(p.lh == 2.5);
    assert!(p.rh == 2.5);

    /* Iterators */
    let natural_numbers: std::ops::RangeFrom<i32> = 1..; // This can be stored in memory because this is computed lazily
    println!("Checking if 1000 is contained in natural numbers...");
    assert!(natural_numbers.contains(&1000));
    println!("Done");
    let my_range: std::ops::Range<i32> = 1..10;
    assert!(my_range.contains(&9));
    assert!(!my_range.contains(&10));
}
