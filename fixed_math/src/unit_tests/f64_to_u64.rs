use crate::{f64_to_u64, max_new_number, Errors};

#[test]
fn test_f64_to_u64() {
    assert_eq!(f64_to_u64(1.23).unwrap(), 123000);
}
#[test]
fn test_f64_to_u64_number_lesser_than_zero() {
    assert_eq!(f64_to_u64(0.009).unwrap(), 900);
}
#[test]
fn test_f64_to_u64_negative_number() {
    let result = f64_to_u64(-1.23);
    assert!(matches!(result, Err(Errors::NegativeNumber)));
}
#[test]
fn test_f64_to_u64_number_too_big() {
    let result = f64_to_u64((max_new_number() + 1) as f64);
    assert!(matches!(result, Err(Errors::NumberTooBig)));
}
