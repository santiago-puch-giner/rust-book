use std::fmt::Display;

pub fn play_with_vectors() {
    // Create an immutable vector from macro
    let vec_macro = vec![0, 1, 2, 3, 4];
    print_vector(&vec_macro);

    // Create a mutable vector and push some values
    let mut pushed_vec: Vec<String> = Vec::new();
    pushed_vec.push(String::from("String 1"));
    pushed_vec.push(String::from("String 2"));
    pushed_vec.push(String::from("String 3"));
    print_vector(&pushed_vec);

    // Can I get an element of the vector "by value"?
    // Yes, if I clone the element
    println!("Clone first element of vector with: `let vec_elem: String = pushed_vec[0].clone();`");
    let vec_elem: String = pushed_vec[0].clone();
    println!("Cloned element: {vec_elem}");
    println!("Vector after clone operation should remain unchanged");
    print_vector(&pushed_vec);

    // Most commonly, you get a reference to such element
    let vec_elem: &String = &pushed_vec[1]; // remember I can re-use the variable name because the previous one is out of scope
    println!("Borrowed element: {vec_elem}");
    println!("Vector after borrow operation should remain unchanged");
    print_vector(&pushed_vec);

    // You can also get a mutable reference and modify it
    println!("Let's get a mutable reference to the element at index 2 and modify it");
    let vec_mut_elem: &mut String = &mut pushed_vec[2];
    *vec_mut_elem = String::from("This is a mutated string hehe");
    print_vector(&pushed_vec);

    // A safer method to get a (mutable) reference to an element or slice of the vector is using the
    // `.get` or `.get_mut` methods
    println!("Let's get a mutable reference again, now using the `.get_mut` method");
    if let Some(val) = pushed_vec.get_mut(0) {
        *val = String::from("This is also a mutated string, noice!");
    }
    print_vector(&pushed_vec);

    // We can iterate over vectors very easily
    let mut count: i8 = 0;
    for val in &vec_macro {
        println!("Iteration {count} -> {val}");
        count += 1;
    }

    // We can also use its iterator with an enumerate option
    for (i, val) in vec_macro.iter().enumerate() {
        println!("Iteration {i} -> {val}");
    }

    // And we can also get a mutable iterator
    let mut mut_vec_macro: Vec<i32> = vec_macro.clone();
    print_vector(&mut_vec_macro);
    for mut_val in &mut mut_vec_macro {
        *mut_val *= *mut_val;
    }
    print_vector(&mut_vec_macro);
}

fn print_vector<T: Display>(vec: &Vec<T>) {
    if vec.is_empty() {
        println!("[]");
        return;
    }

    print!("[");
    for n in 0..vec.len() - 1 {
        print!("{}, ", &vec[n]);
    }
    println!("{}]", &vec[vec.len() - 1]);

    // Cleaner alternative
    // for (i, item) in vec.iter().enumerate() {
    //     ...
    // }
}
