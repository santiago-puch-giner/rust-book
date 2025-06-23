pub fn fibonacci(n: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    let mut next: i32 = b;

    match n {
        0 => 0,
        1 => 1,
        _ => {
            for _ in 1..n {
                next = b + a;
                a = b;
                b = next;
            }
            next
        }
    }
}
