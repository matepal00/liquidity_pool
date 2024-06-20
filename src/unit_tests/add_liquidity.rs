use crate::{Errors, LpPool, Percentage, Price, TokenAmount};

#[test]
fn test_add_liquidity() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    assert!(pool.add_liquidity(TokenAmount(10000000)).is_ok());
}

#[test]
fn test_add_zero_liquidity() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    let result = pool.add_liquidity(TokenAmount(0));
    assert!(matches!(result, Err(Errors::InvalidTokenAmount)));
}
