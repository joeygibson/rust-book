use crate::common::setup;

mod common;

#[test]
fn it_adds_two() {
    setup();
    assert_eq!(adder::add(2, 2), 4);
}