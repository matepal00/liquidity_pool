use crate::{divide, max_dividend, max_divisor, multiplyer, Errors};
#[test]
fn test_divide_basic() {
    assert_eq!(divide(200000, 100000).unwrap(), 200000);
    assert_eq!(divide(1000000, 200000).unwrap(), 500000);
}

#[test]
fn test_divide_by_zero() {
    let result = divide(100000, 0);
    assert!(matches!(result, Err(Errors::DivideByZero)));
}

#[test]
fn test_divide_by_multiplyer() {
    assert_eq!(divide(100000, multiplyer()).unwrap(), 100000);
    assert_eq!(divide(500000, multiplyer()).unwrap(), 500000);
}

#[test]
fn test_divide_same_numbers() {
    assert_eq!(divide(100000, 100000).unwrap(), multiplyer());
    assert_eq!(divide(500000, 500000).unwrap(), multiplyer());
}

#[test]
fn test_divide_max_values() {
    assert_eq!(
        divide(max_dividend(), multiplyer()).unwrap(),
        max_dividend()
    );
}

#[test]
fn test_divide_overflow_dividend() {
    let result = divide(max_dividend() + 1, 100000);
    assert!(matches!(result, Err(Errors::Overflow)));
}

#[test]
fn test_divide_overflow_divisor() {
    let result = divide(100000, max_divisor() + 1);
    assert!(matches!(result, Err(Errors::Overflow)));
}
