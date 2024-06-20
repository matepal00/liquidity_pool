use crate::{max_new_number, max_number_to_multiply, multiply, Errors};
#[test]
fn test_multiply_basic() {
    assert_eq!(multiply(100000, 200000).unwrap(), 200000);
    assert_eq!(multiply(123456, 654321).unwrap(), 807798);
}

#[test]
fn test_multiply_with_zero() {
    assert_eq!(multiply(0, 100000).unwrap(), 0);
    assert_eq!(multiply(100000, 0).unwrap(), 0);
}

#[test]
fn test_multiply_with_one() {
    assert_eq!(multiply(100000, 100000).unwrap(), 100000);
    assert_eq!(multiply(123456, 100000).unwrap(), 123456);
}

#[test]
fn test_multiply_overflow_x_int() {
    let result = multiply(max_number_to_multiply() * 100000 + 1, 100000);
    assert!(matches!(result, Err(Errors::Overflow)));
}

#[test]
fn test_multiply_overflow_y_int() {
    let result = multiply(100000, max_number_to_multiply() + 1);
    assert!(matches!(result, Err(Errors::Overflow)));
}

#[test]
fn test_multiply_overflow_x_int_y_int() {
    let result = multiply(max_new_number() * 100000 + 1, 100000);
    assert!(matches!(result, Err(Errors::Overflow)));
}

#[test]
fn test_multiply_fractional() {
    assert_eq!(multiply(123456789, 987654321).unwrap(), 1219326311126);
}

#[test]
fn test_multiply_large_fractional() {
    assert_eq!(multiply(999999999, 999999999).unwrap(), 9999999980000);
}
