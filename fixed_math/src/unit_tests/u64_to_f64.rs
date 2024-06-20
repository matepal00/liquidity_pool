use crate::u64_to_f64;
#[test]
fn test_u64_to_f64() {
    assert_eq!(u64_to_f64(123000), 1.23);
}
#[test]
fn test_u64_to_f64_number_lesser_than_zero() {
    assert_eq!(u64_to_f64(12), 0.00012);
}
