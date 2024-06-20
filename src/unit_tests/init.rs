use crate::{Errors, LpPool, Percentage, Price, TokenAmount};

#[test]
fn test_init() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let pool: Result<LpPool, Errors> = LpPool::init(price, min_fee, max_fee, liquidity_target);
    assert!(pool.is_ok());
}

#[test]
fn test_init_price_zero() {
    let price: Price = Price(0);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let pool: Result<LpPool, Errors> = LpPool::init(price, min_fee, max_fee, liquidity_target);
    assert!(matches!(pool, Err(Errors::InvalidInitialValues)));
}

#[test]
fn test_init_with_zero_liquidity_target() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(0);
    let pool: Result<LpPool, Errors> = LpPool::init(price, min_fee, max_fee, liquidity_target);
    assert!(matches!(pool, Err(Errors::InvalidInitialValues)));
}

#[test]
fn test_init_min_fee_greater_than_max_fee() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(900000);
    let max_fee: Percentage = Percentage(10000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let pool: Result<LpPool, Errors> = LpPool::init(price, min_fee, max_fee, liquidity_target);
    assert!(matches!(pool, Err(Errors::InvalidInitialValues)));
}
