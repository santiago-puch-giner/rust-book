mod box_pointer;
mod deref;
mod drop;

fn main() {
    println!("Hello, world!");
    box_pointer::demo();
    deref::demo();
    drop::demo();
}
