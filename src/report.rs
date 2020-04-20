use std::fmt;

pub struct BalanceSheet {
    assets: f64,
    current_assets: f64,
    goodwill: f64,
    fixed_assets: f64,
    liabilities: f64,
    current_liabilities: f64,
    long_term_debt: f64,
    equity: f64,
}

impl BalanceSheet {
    pub fn new(
        assets: f64,
        current_assets: f64,
        goodwill: f64,
        fixed_assets: f64,
        liabilities: f64,
        current_liabilities: f64,
        long_term_debt: f64,
        equity: f64,
    ) -> Result<Self, BalanceMismatchError> {
        if assets != equity + liabilities {
            return Err(BalanceMismatchError {});
        }
        Ok(BalanceSheet {
            assets,
            current_assets,
            goodwill,
            fixed_assets,
            liabilities,
            current_liabilities,
            long_term_debt,
            equity,
        })
    }
}

#[derive(Debug)]
pub struct BalanceMismatchError {}

impl fmt::Display for BalanceMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Total Assets not equal Liabilities and Equity")
    }
}
