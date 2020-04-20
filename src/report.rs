use std::fmt;

pub struct IncomeStatement {
    revenue: f64,
    cost_of_revenue: f64,
    gross_profit: f64,
    operating_expense: f64,
    income_from_operations: f64,
    income_before_taxes: f64,
    net_income: f64,
}

impl IncomeStatement {
    pub fn new(
        revenue: f64,
        cost_of_revenue: f64,
        gross_profit: f64,
        operating_expense: f64,
        income_from_operations: f64,
        income_before_taxes: f64,
        net_income: f64,
    ) -> Self {
        IncomeStatement {
            revenue,
            cost_of_revenue,
            gross_profit,
            operating_expense,
            income_from_operations,
            income_before_taxes,
            net_income,
        }
    }
}

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
