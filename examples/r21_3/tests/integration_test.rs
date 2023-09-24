mod common;
mod tools;

#[test]
fn test_add() {
    assert_eq!(r21_3::add(2, 3), 5);
}

#[test]
fn test_add_v2() {
    common::setup();
    assert_eq!(r21_3::add(3, 2), 5);
}