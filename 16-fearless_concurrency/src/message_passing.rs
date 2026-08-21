use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn simple_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got {}", received);
}

pub fn concurrent_message_sending() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "a".to_string(),
            "different".to_string(),
            "thread".to_string(),
        ];
        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("{}", received);
    }
}
