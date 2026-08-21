mod message_passing;
mod simple_thread;

fn main() {
    simple_thread::simple_thread();
    simple_thread::simple_thread_with_move_closure();
    message_passing::simple_example();
    message_passing::concurrent_message_sending();
}
