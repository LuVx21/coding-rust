pub mod common;

use coding_common::math;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(math::add(2, 3), 5);
}