// Generics for structs
struct Point<T> {
    x: T,
    y: T,
}

// Generics for enums
#[derive(Debug)]
enum TernaryOption<U, V> {
    Option1(U),
    Option2(V),
    None,
}

// Generics for methods (impl of struct)
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Example of how generics enables re-using the same logic for compatible types
fn largest_number(list: &[i32]) -> &i32 {
    // This is a mutable variable that holds an immutable reference
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            // The immutable reference is changed here
            largest = number;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn main() {
    // Structs
    let point_float = Point { x: 1.2, y: 2.3 };
    let point_int = Point { x: -1, y: -20 };
    let point_uint8: Point<u8> = Point { x: 1, y: 255 };
    let point_bool = Point { x: true, y: false }; // even if it does not make logical sense, it is possible

    // Enums
    let mut option_1: TernaryOption<f32, i32> = TernaryOption::Option1(1.4);
    println!("Option 1 is now {:?}", option_1);
    option_1 = TernaryOption::Option2(-192);
    println!("Option 1 is now {:?}", option_1);
    option_1 = TernaryOption::None;
    println!("Option 1 is now {:?}", option_1);

    // Methods
    println!("x for point_float = {}", point_float.x());
    println!("x for point_int = {}", point_int.x());
    println!("x for point_uint8 = {}", point_uint8.x());
    println!("x for point_bool = {}", point_bool.x());

    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_number(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_number(&number_list);
    println!("The largest number is {result}");

    // Using the generic function
    // let result_generic: &i32 = largest(&number_list);
    // println!("The largest number is {result_generic}");

    // let float_list = vec![-1.32, 0.43, 102.53, 102319.32];
    // let result_float: &f32 = largest(&float_list);
    // println!("The largest number is {result_float}");
}
