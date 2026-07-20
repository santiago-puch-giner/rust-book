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

// Using generic types we can re-use the same logic for any compatible type
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_number(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_number(&number_list);
    println!("The largest number is {result}");

    // Using the generic function
    let result_generic: &i32 = largest(&number_list);
    println!("The largest number is {result_generic}");

    let float_list = vec![-1.32, 0.43, 102.53, 102319.32];
    let result_float: &f32 = largest(&float_list);
    println!("The largest number is {result_float}");
}
