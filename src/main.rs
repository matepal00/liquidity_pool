#[cfg(test)]
mod unit_tests;
use fixed_math::{add, divide, f64_to_u64, multiply, u64_to_f64};

#[derive(Debug, PartialEq)]
struct TokenAmount(u64);

#[derive(Debug, PartialEq)]
struct StakedTokenAmount(u64);

#[derive(Debug)]
struct LPTokenAmount(u64);

#[derive(Debug)]
struct Price(u64);

#[derive(Debug)]
struct Percentage(u64);

#[derive(Debug)]
struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LPTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}
#[derive(Debug)]
pub enum Errors {
    InvalidInitialValues,
    InvalidTokenAmount,
    NotEnoughTokensInLiquidityPool,
    InvalidLPTokenAmount,
    InvalidSTTokenAmount,
}
impl LpPool {
    fn init(
        price: Price,
        min_fee: Percentage,
        max_fee: Percentage,
        liquidity_target: TokenAmount,
    ) -> Result<Self, Errors> {
        if price.0 > 0 && liquidity_target.0 > 0 && min_fee.0 <= max_fee.0 {
            return Ok(LpPool {
                price,
                token_amount: TokenAmount(0),
                st_token_amount: StakedTokenAmount(0),
                lp_token_amount: LPTokenAmount(0),
                liquidity_target,
                min_fee,
                max_fee,
            });
        }
        return Err(Errors::InvalidInitialValues);
    }

    fn add_liquidity(&mut self, token_amount: TokenAmount) -> Result<LPTokenAmount, Errors> {
        if token_amount.0 == 0 {
            return Err(Errors::InvalidTokenAmount);
        }
        let lp_tokens_to_mint: u64 = if self.lp_token_amount.0 == 0 {
            token_amount.0
        } else {
            divide(
                multiply(token_amount.0, self.lp_token_amount.0).unwrap(),
                add(
                    self.token_amount.0,
                    multiply(self.st_token_amount.0, self.price.0).unwrap(),
                )
                .unwrap(),
            )
            .unwrap()
        };
        self.token_amount.0 = add(self.token_amount.0, token_amount.0).unwrap();
        self.lp_token_amount.0 = add(self.lp_token_amount.0, lp_tokens_to_mint).unwrap();
        return Ok(LPTokenAmount(lp_tokens_to_mint));
    }

    fn remove_liquidity(
        &mut self,
        lp_token_amount: LPTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        if lp_token_amount.0 == 0 || lp_token_amount.0 > self.lp_token_amount.0 {
            return Err(Errors::InvalidLPTokenAmount);
        }
        let token_to_return_amount: u64 = multiply(
            divide(lp_token_amount.0, self.lp_token_amount.0).unwrap(),
            self.token_amount.0,
        )
        .unwrap();
        let staked_token_to_return_amount: u64 = multiply(
            divide(lp_token_amount.0, self.lp_token_amount.0).unwrap(),
            self.st_token_amount.0,
        )
        .unwrap();
        self.token_amount.0 -= token_to_return_amount;
        self.st_token_amount.0 -= staked_token_to_return_amount;
        self.lp_token_amount.0 -= lp_token_amount.0;
        return Ok((
            TokenAmount(token_to_return_amount),
            StakedTokenAmount(staked_token_to_return_amount),
        ));
    }

    fn swap(&mut self, st_token_amount: StakedTokenAmount) -> Result<TokenAmount, Errors> {
        if st_token_amount.0 == 0 {
            return Err(Errors::InvalidSTTokenAmount);
        }
        let token_amount: u64 = multiply(st_token_amount.0, self.price.0).unwrap();

        if token_amount > self.token_amount.0 {
            return Err(Errors::NotEnoughTokensInLiquidityPool);
        }
        let token_amount_after_swap: u64 = self.token_amount.0 - token_amount;
        let fee: u64 = if self.token_amount.0 - token_amount < self.liquidity_target.0 {
            self.max_fee.0
                - multiply(
                    self.max_fee.0 - self.min_fee.0,
                    divide(token_amount_after_swap, self.liquidity_target.0).unwrap(),
                )
                .unwrap()
        } else {
            self.min_fee.0
        };
        let token_to_return_amount: u64 =
            token_amount - (multiply(token_amount, fee).unwrap() / 100);
        self.token_amount.0 -= token_to_return_amount;
        self.st_token_amount.0 = add(self.st_token_amount.0, st_token_amount.0).unwrap();
        return Ok(TokenAmount(token_to_return_amount));
    }
}

fn main() {
    let price: f64 = 1.5;
    let min_fee: f64 = 0.1;
    let max_fee: f64 = 9.0;
    let liquidity_target: f64 = 90.0;
    let lp_pool: Result<LpPool, Errors> = LpPool::init(
        Price(f64_to_u64(price).unwrap()),
        Percentage(f64_to_u64(min_fee).unwrap()),
        Percentage(f64_to_u64(max_fee).unwrap()),
        TokenAmount(f64_to_u64(liquidity_target).unwrap()),
    );
    match lp_pool {
        Ok(mut pool) => {
            println!("Liquidity pool initialized!");
            let liquidity_to_add: f64 = 100.0;
            match pool.add_liquidity(TokenAmount(f64_to_u64(liquidity_to_add).unwrap())) {
                Ok(lp_token_amount) => println!(
                    "You successfully added a liquidity! Your LP Tokens in return: {:?}",
                    u64_to_f64(lp_token_amount.0)
                ),
                Err(err) => println!("There was an error during addition of liquidity: {:?}", err),
            }
            let staked_tokens_to_exchange: f64 = 6.0;
            match pool.swap(StakedTokenAmount(
                f64_to_u64(staked_tokens_to_exchange).unwrap(),
            )) {
                Ok(token_amount) => println!(
                    "You successfully exchanged your staked tokens! Your Tokens in return: {:?}",
                    u64_to_f64(token_amount.0)
                ),
                Err(err) => println!("There was an error during swap: {:?}", err),
            }
            let liquidity_to_add: f64 = 10.0;
            match pool.add_liquidity(TokenAmount(f64_to_u64(liquidity_to_add).unwrap())) {
                Ok(lp_token_amount) => println!(
                    "You successfully added a liquidity! Your LP Tokens in return: {:?}",
                    u64_to_f64(lp_token_amount.0)
                ),
                Err(err) => println!("There was an error during addition of liquidity: {:?}", err),
            }
            let staked_tokens_to_exchange: f64 = 30.0;
            match pool.swap(StakedTokenAmount(
                f64_to_u64(staked_tokens_to_exchange).unwrap(),
            )) {
                Ok(token_amount) => println!(
                    "You successfully exchanged your staked tokens! Your Tokens in return: {:?}",
                    u64_to_f64(token_amount.0)
                ),
                Err(err) => println!("There was an error during swap: {:?}", err),
            }
            let liquidity_to_remove: f64 = 109.9991;
            match pool.remove_liquidity(LPTokenAmount(f64_to_u64(liquidity_to_remove).unwrap())){
                Ok((token_amount, st_token_amount)) => println!(
                    "You successfully removed a liquidity! Your Tokens in return: {:?} Token and {:?} Staked Token",
                    u64_to_f64(token_amount.0),u64_to_f64(st_token_amount.0)),
                Err(err) => println!("There was an error during removal of liquidity: {:?}",err),
            }
        }
        Err(err) => println!(
            "There was an error during liquidity pool initialization: {:?}",
            err
        ),
    }
}
