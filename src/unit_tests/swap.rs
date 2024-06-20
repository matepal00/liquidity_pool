use crate::{LpPool, Percentage, Price, StakedTokenAmount, TokenAmount};

#[test]
fn test_swap() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    let _ = pool.add_liquidity(TokenAmount(10000000));
    assert_eq!(pool.swap(StakedTokenAmount(600000)).unwrap().0, 899100);
}
#[test]
fn test_swap_with_zero_st_tokens() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    let _ = pool.add_liquidity(TokenAmount(10000000));
    let result = pool.swap(StakedTokenAmount(0));
    assert!(matches!(result, Err(crate::Errors::InvalidSTTokenAmount)));
}

#[test]
fn test_swap_with_not_enough_liquidity() {
    let price: Price = Price(150000);
    let min_fee: Percentage = Percentage(10000);
    let max_fee: Percentage = Percentage(900000);
    let liquidity_target: TokenAmount = TokenAmount(9000000);
    let mut pool: LpPool = LpPool::init(price, min_fee, max_fee, liquidity_target).unwrap();
    let result = pool.swap(StakedTokenAmount(8000000));
    assert!(matches!(
        result,
        Err(crate::Errors::NotEnoughTokensInLiquidityPool)
    ));
}
