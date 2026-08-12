use std::ops::Drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn void(_data: CustomSmartPointer) {
    // Do nothing, takes ownership and then drops the value
}

pub fn demo() {
    let _c = CustomSmartPointer {
        data: String::from("c: data for my smart pointer"),
    };
    let d = CustomSmartPointer {
        data: String::from("d: more data for my other smart pointer"),
    };
    let _e = CustomSmartPointer {
        data: String::from("e: dataaaa"),
    };
    let f = CustomSmartPointer {
        data: String::from("f: alrighty..."),
    };
    println!("Calling sink on smart pointer `c`");
    void(d); // we can't call the `.drop` method directly, explicit destructor calls are not allowed
    drop(f); // but we can call the std lib `drop` function
    println!("Ending demo function...");

    // note how first `d` is dropped because of the `void` function,
    // then f because we call the explicit destructor `drop`,
    // then e and then d because... it's a stack!
}
