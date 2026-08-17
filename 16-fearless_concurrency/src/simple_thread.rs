use std::thread;
use std::time;

pub fn simple_thread() {
    let thread_handler = thread::spawn(|| {
        for i in 1..=10 {
            println!("Count from spawned thread: {}", i);
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    for i in 1..=5 {
        println!("Count from main thread: {}", i);
        thread::sleep(time::Duration::from_secs(1));
    }

    thread_handler.join().unwrap();
}

pub fn simple_thread_with_move_closure() {
    let v = vec![1, 2, 3, 4];

    // We need to force move ownership to the thread
    // If we didn't we could `drop(v)` while the thread is running and that would lead to undefined behaviour
    thread::spawn(move || {
        // move captures a closure's environment by value
        println!("Here's a vector: {:?}", v);
    });
}
