use crate::{LPTokenAmount, LpPool, Percentage, Price, StakedTokenAmount, TokenAmount};

#[test]
fn test_remove_liquidity() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    let _ = pool.add_liquidity(TokenAmount(10000000));
    assert_eq!(
        pool.remove_liquidity(LPTokenAmount(10000000)).unwrap(),
        (TokenAmount(10000000), StakedTokenAmount(0))
    );
}

#[test]
fn test_remove_liquidity_with_zero_lp_tokens() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    let _ = pool.add_liquidity(TokenAmount(10000000));
    let result = pool.remove_liquidity(LPTokenAmount(0));
    assert!(matches!(result, Err(crate::Errors::InvalidLPTokenAmount)));
}

#[test]
fn test_remove_more_liquidity_than_available() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    let _ = pool.add_liquidity(TokenAmount(10000000));
    let result = pool.remove_liquidity(LPTokenAmount(10000001));
    assert!(matches!(result, Err(crate::Errors::InvalidLPTokenAmount)));
}
