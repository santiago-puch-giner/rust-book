use std::fmt::{Debug, Display};

// Generics for structs
struct Point<T> {
    x: T,
    y: T,
}

// Generics for enums
#[derive(Debug)]
enum TernaryOption<U, V> {
    Option1(U),
    Option2(V),
    None,
}

// Generics for methods (impl of struct)
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

// Traits allow us to define a set of behaviours necessary to accomplish some purpose
trait Summary {
    fn summarize_author(&self) -> String;

    // We can also have default implementations for trait methods
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    #[allow(dead_code)]
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!(
            "{}: {} [replied={}, reposted={}]",
            self.username, self.content, self.reply, self.repost
        )
    }
}

// Trait bound syntax
#[allow(dead_code)]
fn notification<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Syntactic sugar for trait bound syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
#[allow(dead_code)]
fn summarize(item: &(impl Summary + Display)) {
    println!("Summary: {}", item.summarize());
}
// The equivalent with trait bounds is valid:
// pub fn notify<T: Summary + Display>(item: &T)
#[allow(dead_code)]
#[allow(unused_variables)]
fn function_where_traits<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// Returning types that implement traits
#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

// Trait implementation for generics
impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Generics + traits to re-use logic
fn largest_number(list: &[i32]) -> &i32 {
    // This is a mutable variable that holds an immutable reference
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            // The immutable reference is changed here
            largest = number;
        }
    }

    largest
}

// Now we can use this function for any type that implements partial ordering
// For example, any number, char, etc.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Blanket implementations
// For example, here we are implementing the ToStringV2 trait on any type that implements the Display trait
trait ToStringV2 {
    fn to_string_v2(&self) -> String;
}

impl<T: Display> ToStringV2 for T {
    fn to_string_v2(&self) -> String {
        format!("{}", &self)
    }
}

struct Node {
    name: String,
    id: u64,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id={})", self.name, self.id)
    }
}

struct Edge {
    name: String,
    id: u64,
    src_id: u64,
    dst_id: u64,
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})-[{}]->({}) (id={})",
            self.src_id, self.name, self.dst_id, self.id
        )
    }
}

fn main() {
    // Structs
    let point_float = Point { x: 1.2, y: 2.3 };
    let point_int = Point { x: -1, y: -20 };
    let point_uint8: Point<u8> = Point { x: 1, y: 255 };
    let point_bool = Point { x: true, y: false }; // even if it does not make logical sense, it is possible
    let point_char = Point::new('a', 'b');

    // Enums
    let mut option_1: TernaryOption<f32, i32> = TernaryOption::Option1(1.4);
    println!("Option 1 is now {:?}", option_1);
    option_1 = TernaryOption::Option2(-192);
    println!("Option 1 is now {:?}", option_1);
    option_1 = TernaryOption::None;
    println!("Option 1 is now {:?}", option_1);

    // Methods
    println!("x for point_float = {}", point_float.x());
    println!("x for point_int = {}", point_int.x());
    println!("x for point_uint8 = {}", point_uint8.x());
    println!("x for point_bool = {}", point_bool.x());

    // Traits and traits with generics
    let news_1 = NewsArticle {
        headline: String::from("Something happened in Munich"),
        location: String::from("Munich"),
        author: String::from("Jonas Welke"),
        content: String::from("Yesterday something happened"),
    };
    let social_post_1 = SocialPost {
        username: String::from("jonas.welke"),
        content: String::from("Yesterday something happened"),
        reply: true,
        repost: true,
    };
    notify(&news_1);
    notify(&social_post_1);

    // Using the implementation of Point<T> with trait bounds (Display + PartialOrd)
    point_float.cmp_display();
    point_bool.cmp_display();
    point_uint8.cmp_display();
    point_char.cmp_display();

    // Using generics to avoid code duplication
    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_number(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_number(&number_list);
    println!("The largest number is {result}");

    // Using the generic function with trait PartialOrd bound to the generic type T
    let result_generic: &i32 = largest(&number_list);
    println!("The largest number is {result_generic}");

    let float_list = vec![-1.32, 0.43, 102.53, 102319.32];
    let result_float: &f32 = largest(&float_list);
    println!("The largest number is {result_float}");

    // Nodes and Edges with the ToStringV2 trait
    let node_1 = Node {
        name: String::from("Santi"),
        id: 1,
    };
    let node_2 = Node {
        name: String::from("Rust"),
        id: 2,
    };
    let edge = Edge {
        name: String::from("LEARNS"),
        src_id: node_1.id,
        dst_id: node_2.id,
        id: 3,
    };
    let node_1_str = node_1.to_string_v2();
    let node_2_str = node_2.to_string_v2();
    let edge_str = edge.to_string_v2();
    let subgraph_str = format!("nodes={} {} / edges={}", node_1_str, node_2_str, edge_str);
    print!("{subgraph_str}");
}
