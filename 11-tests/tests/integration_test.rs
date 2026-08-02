use tests::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(add_two(1), 3);
}
