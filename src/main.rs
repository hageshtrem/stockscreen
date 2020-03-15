use serde::{Deserialize, Serialize};

const URL: &'static str = r"https://query2.finance.yahoo.com/v10/finance/quoteSummary/MOEX.ME?formatted=true&crumb=th3yI76E9D%2F&lang=en-US&region=US&modules=incomeStatementHistory%2CcashflowStatementHistory%2CbalanceSheetHistory%2CincomeStatementHistoryQuarterly%2CcashflowStatementHistoryQuarterly%2CbalanceSheetHistoryQuarterly&corsDomain=finance.yahoo.com";

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Response {
    quoteSummary: QuoteSummary,
}

#[derive(Serialize, Deserialize, Debug)]
struct QuoteSummary {
    result: Vec<ReportResult>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct ReportResult {
    incomeStatementHistory: IncomeStatementHistory,
    incomeStatementHistoryQuarterly: IncomeStatementHistory,
    // balanceSheetHistory: BalanceSheetHistory,
    // balanceSheetHistoryQuarterly: BalanceSheetHistory,
    // cashflowStatementHistory:
    // cashflowStatementHistoryQuarterly:
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct IncomeStatementHistory {
    incomeStatementHistory: Vec<IncomeStatementHistoryElement>,
    maxAge: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct IncomeStatementHistoryElement {
    maxAge: u32,
    endDate: Value,
    totalRevenue: Value,
    costOfRevenue: Value,
    grossProfit: Value,
    researchDevelopment: Option<Value>,
    sellingGeneralAdministrative: Value,
    nonRecurring: Option<Value>,
    otherOperationExpenses: Option<Value>,
    totalOperationExpenses: Option<Value>,
    operatingIncome: Value,
    totalOtherIncomeExpenseNet: Value,
    ebit: Value,
    interestExpense: Value,
    incomeBeforeTax: Value,
    incomeTaxExpense: Value,
    minorityInterest: Value,
    netIncomeFromContinuingOps: Value,
    discontinuedOperations: Option<Value>,
    extraordinaryItems: Option<Value>,
    effectOfAccountingChanges: Option<Value>,
    otherItems: Option<Value>,
    netIncome: Value,
    netIncomeApplicableToCommonShares: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Value {
    raw: Option<i64>,
    fmt: Option<String>,
    longFmt: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?.json::<Response>().await?;
    println!("{:#?}", resp);
    Ok(())
}
