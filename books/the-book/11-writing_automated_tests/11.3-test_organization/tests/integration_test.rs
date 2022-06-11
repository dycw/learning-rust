// The tests Directory

use test_organization;

#[test]
fn it_adds_two_1() {
    assert_eq!(4, test_organization::add_two(2));
}

// Submodules in Integration Tests

mod common_2;

#[test]
fn it_adds_two_2() {
    common_2::setup();
    assert_eq!(4, test_organization::add_two(2));
}
