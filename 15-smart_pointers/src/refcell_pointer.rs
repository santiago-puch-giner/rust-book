/*
 * RECAP
 * - Rc<T> enables multiple owners of the same data;
 *  - Box<T> and RefCell<T> have single owners.
 * - Box<T> allows immutable or mutable borrows checked at compile time;
 *  - Rc<T> allows only immutable borrows checked at compile time;
 *  - RefCell<T> allows immutable or mutable borrows checked at runtime.
 * - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T>
 * even when the RefCell<T> is immutable.
 */

use std::cell::RefCell;
use std::rc::Rc;

// Interior mutability pattern: mutating the value inside an immutable value

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        // We need interior mutability here so that we can store data from an immutable ref
        // sent_messages: Vec<String>, <- this does not work
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // We can only mutate our own attribute `sent_messages` from an immutable reference to self `&self`
            // thanks to interior mutability
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 100);
        tracker.set_value(79);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Multiple owners of mutable data
// Combining Rc<T> and RefCell<T> -> Rc<RefCell<T>>
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn demo() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
