use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(s: &str) {
    println!("Hello from {s}");
}

pub fn demo() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // only possible because of the Deref trait
    assert_eq!(5, *(y.deref())); // what Rust calls for the line above

    // Example of Automatic Deref Coercion
    let m = MyBox(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str
    hello(&(*m)[..]); // this would be the manual deref equivalent to the chain above

    /*
     * Deref coercion supported by Rust
     *
     * - From &T to &U when T: Deref<Target=U>
     * - From &mut T to &mut U when T: DerefMut<Target=U>
     * - From &mut T to &U when T: Deref<Target=U>
     */
}
