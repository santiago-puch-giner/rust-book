mod box_pointer;
mod deref;
mod drop;
mod rc_pointer;
mod refcell_pointer;
mod weak_pointer;

fn print_separator(newline: bool) {
    println!("-----------------");
    if newline {
        println!("");
    }
}

fn main() {
    println!("Box<T>");
    print_separator(false);
    box_pointer::demo();
    print_separator(true);

    println!("Deref Trait");
    print_separator(false);
    deref::demo();
    print_separator(true);

    println!("Drop Trait");
    print_separator(false);
    drop::demo();
    print_separator(true);

    println!("Rc<T>");
    print_separator(false);
    rc_pointer::demo();
    print_separator(true);

    println!("RefCell<T>");
    print_separator(false);
    refcell_pointer::demo();
    print_separator(true);

    println!("Weak<T>");
    print_separator(false);
    weak_pointer::demo();
    print_separator(true);
}
