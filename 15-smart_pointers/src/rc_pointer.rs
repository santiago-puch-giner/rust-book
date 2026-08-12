use std::rc::Rc;

// Reference Counter is a smart pointer that allows multiple ownership by keeping a count
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn demo() {
    let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let _b = Cons(4, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
        let _c = Cons(5, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));
}
