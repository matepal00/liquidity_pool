use crate::{add, max_number_to_add, Errors};

#[test]
fn add_test() {
    assert_eq!(5, add(2, 3).unwrap());
}
#[test]
fn add_test_x_to_big() {
    let result = add(max_number_to_add() + 1, 1);
    assert!(matches!(result, Err(Errors::Overflow)));
}
#[test]
fn add_test_y_to_big() {
    let result = add(1, max_number_to_add() + 1);
    assert!(matches!(result, Err(Errors::Overflow)));
}
