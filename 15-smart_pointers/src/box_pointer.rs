// Recursive data types from LISP
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn demo() {
    // let's create a cons list
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let mut current_cell = &list;

    while let Cons(v, next_cell) = current_cell {
        println!("current cons cell value: {v}");
        current_cell = next_cell;
    }
    println!("end of cons");
}
