fn main() {
    /* CLOSURES */
    let closure_captured = 1;

    fn add_one_v1(x: u32, context: u32) -> u32 {
        x + context + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + closure_captured + 1 };
    let add_one_v3 = |x| x + closure_captured + 1;
    let add_one_v4 = |x| x + closure_captured + 1;

    let result_v1 = add_one_v1(1, closure_captured);
    let result_v2 = add_one_v2(1);
    let result_v3 = add_one_v3(1);
    let result_v4 = add_one_v4(1);

    assert_eq!(result_v1, result_v2);
    assert_eq!(result_v1, result_v3);
    assert_eq!(result_v1, result_v4);

    /* ITERATORS */
    let vec = vec![1, 2, 3, 4];
    let vec_iter = vec.iter();

    for v in vec_iter {
        println!("{}", v);
    }

    // Calling the .next() method
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // mutable since the iterator is consumed, aka its internal state changes
    assert_eq!(v1_iter.next(), Some(&1));

    // Example method .sum() implemented by the Iterator trait
    // This is a consuming adapter that calls .next() internally
    // Takes ownership of the iterator and consumes it
    let v2 = vec![1, 2, 3, 5, 8];
    let v2_sum: i32 = v2.iter().sum();
    println!("Sum of {:?}: {}", v2, v2_sum);

    // Example method .map() implemented by the Iterator trait
    // This is an iterator adapter, which unlike a consuming adapter, does not consume the iterator
    let v3 = vec![-1, -2, -3, -4];
    for v in v3.iter().map(|x| x * 2) {
        println!("Double value: {}", v)
    }

    // Iterators are lazy, thus they have to be consumed.
    // One way to do that is to collect them into a new Vec
    let v3_map: Vec<i32> = v3.iter().map(|x| x * -1).collect();
    println!("Transformed vec: {:?}", v3_map);

    let v3_owned_iter = v3.into_iter();
    // println!("{:?}", v3); <-- this does not compile
    for v in v3_owned_iter {
        println!("{}", v);
    }
}
