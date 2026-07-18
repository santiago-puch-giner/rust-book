use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // You can uncomment these lines to make the program panic

    // panic!("Forced panic!");

    // let v = vec![10];
    // v[10];

    // Using Result<T, E>
    let greeting_file = File::open("hello.txt");

    let greeting_file_fd = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };
    let greeting_metadata = greeting_file_fd.metadata().expect("expected file metadata");
    println!("File len: {}", greeting_metadata.len());

    // Use of expect or unwrap
    // `expect` is more idiomatic because you can write the message of the panic, but `unwrap` is used too.
    let saludo_file = File::open("hola.txt").expect("hola.txt should be included in this project");
    let saludo_metadata = saludo_file.metadata().expect("expected file metadata");
    println!("File len: {}", saludo_metadata.len());

    // Propagating errors with the `?` operator
    let username = read_username_from_file()
        .expect("Expected username to be read from username.txt at this point");
    println!("Username: {username}");
    // Also options can be propagated with the `?` operator
    println!("{}", last_char_of_first_line(&username).unwrap_or(' '));

    // Given that main returns Result<(), Box<dyn Error>>, we can use the `?`
    // operator here
    read_username_from_file()?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // could be implemented with a single line
    // fs::read_to_string("username.txt")
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
