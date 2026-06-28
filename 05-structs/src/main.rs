/**
 * Structs hold multiple related values of different types (like tuples)
 * where each piece of data is named and does not have to be written / accessed in a specific order.
**/

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    /**
     * Functions within an implementation block are associated functions
     * If they take a &self, they are also called methods, and called with the dot notation: instance.method()
     * The ones that do not take a &self are usually used for constructors. The idiomatic name is `new`.
     **/

    // Methods
    fn more_active_than(&self, other: &User) -> bool {
        self.sign_in_count > other.sign_in_count
    }

    fn eq(&self, other: &User) -> bool {
        self.username == other.username
            && self.email == other.email
            && self.active == other.active
            && self.sign_in_count == other.sign_in_count
    }

    fn set_username(&mut self, username: &str) {
        self.username = username.to_owned();
    }

    // Associative function
    // Field init shorthand: when fn params match struct's fields you don't need to repeat them
    fn new(username: String, email: String, active: Option<bool>) -> User {
        User {
            username,
            email,
            active: active.unwrap_or(false),
            sign_in_count: 0,
        }
    }
}

/** Tuple Structs **/
#[derive(Debug)]
struct Point(i32, i32, i32);

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

impl Color {
    fn add_brightness(&self, brightness: i32) -> Color {
        Color(
            self.0 + brightness,
            self.1 + brightness,
            self.2 + brightness,
        )
    }
}

/** Unit-like Structs **/
#[derive(Debug)]
struct Empty;

fn main() {
    let user = User {
        username: "mike04".to_string(),
        email: "mike04@mail.com".to_string(),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user);

    let mut user_2 = User::new(
        String::from("lana1239"),
        String::from("lana1239@mail.me"),
        Some(true),
    );
    user_2.sign_in_count = 4;
    println!("{:?}", user_2);

    println!(
        "Is {} more active than {}: {}",
        user.username,
        user_2.username,
        user.more_active_than(&user_2)
    );
    println!(
        "Is {} more active than {}: {}",
        user_2.username,
        user.username,
        user_2.more_active_than(&user)
    );

    println!(
        "Is user {:?} the same as {:?}? {}",
        user.username,
        user_2.username,
        user.eq(&user_2)
    );
    let user_copy = User {
        username: String::from("mike04"),
        email: String::from("mike04@mail.com"),
        ..user
    };
    println!(
        "Is user {:?} the same as {:?}? {}",
        user.username,
        user_copy.username,
        user.eq(&user_copy)
    );

    // Using tuple structs
    let point_1 = Point(0, 1, -1);
    let point_2 = Point(3, 10, 20);
    let color_1 = Color(129, 13, 96);
    let color_2 = Color(3, 10, 20);

    let point_3 = point_1.add(&point_2);
    println!("{:?} + {:?} = {:?}", point_1, point_2, point_3);

    // even thought they are both tuple-like with 3 int32 numbers, I can't pass the Color type to this function
    // let color_3 = point_1.add(&color_2);
    // let color_3 = color_1.add(&color_2);
    println!("Colors can't be summed: {:?} + {:?}", color_1, color_2);
    let brightness = 10;
    let color_1_brighter = color_1.add_brightness(10);
    println!(
        "But they can be made brighter!: {:?} + {:?} -> {:?}",
        color_1, brightness, color_1_brighter
    );

    // empty struct
    let empty = Empty;
    println!("{:?}", empty);

    // Accessing methods through pointers. Unlike C++, no distinction between a.method() or a->method()
    let mut boxed_user: Box<User> = Box::new(User::new(
        "jisos".to_string(),
        "jisos_da_great@gmail.com".to_string(),
        Some(true),
    ));
    boxed_user.set_username("jisos_da_great"); // <-- here!
    println!("{:?}", boxed_user);
}
