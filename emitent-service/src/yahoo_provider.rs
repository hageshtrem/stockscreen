use serde::{Deserialize, Serialize};
use std::fmt;

static CHROME_USER_AGENT: &str = r"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.87 Safari/537.36";

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
    balanceSheetHistory: BalanceSheetHistory,
    balanceSheetHistoryQuarterly: BalanceSheetHistory,
    cashflowStatementHistory: CashflowStatementHistory,
    cashflowStatementHistoryQuarterly: CashflowStatementHistory,
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
    otherOperatingExpenses: Value,
    totalOperatingExpenses: Value,
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
    effectOfAccountingCharges: Option<Value>,
    otherItems: Option<Value>,
    netIncome: Value,
    netIncomeApplicableToCommonShares: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct BalanceSheetHistory {
    balanceSheetStatements: Vec<BalanceSheetStatement>,
    maxAge: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct BalanceSheetStatement {
    maxAge: u32,
    endDate: Value,
    cash: Value,
    shortTermInvestments: Option<Value>,
    netReceivables: Value,
    inventory: Option<Value>,
    otherCurrentAssets: Value,
    totalCurrentAssets: Value,
    longTermInvestments: Option<Value>,
    propertyPlantEquipment: Value,
    goodWill: Option<Value>,
    intangibleAssets: Option<Value>,
    otherAssets: Value,
    deferredLongTermAssetCharges: Option<Value>,
    totalAssets: Value,
    accountsPayable: Value,
    shortLongTermDebt: Option<Value>,
    otherCurrentLiab: Value,
    longTermDebt: Option<Value>,
    otherLiab: Value,
    minorityInterest: Value,
    totalCurrentLiabilities: Value,
    totalLiab: Value,
    commonStock: Value,
    retainedEarnings: Option<Value>,
    treasuryStock: Option<Value>,
    capitalSurplus: Option<Value>,
    otherStockholderEquity: Option<Value>,
    totalStockholderEquity: Value,
    netTangibleAssets: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct CashflowStatementHistory {
    cashflowStatements: Vec<CashflowStatement>,
    maxAge: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct CashflowStatement {
    maxAge: u32,
    endDate: Value,
    netIncome: Value,
    depreciation: Option<Value>,
    changeToNetincome: Option<Value>,
    changeToAccountReceivables: Option<Value>,
    changeToLiabilities: Option<Value>,
    changeToOperatingActivities: Option<Value>,
    totalCashFromOperatingActivities: Option<Value>,
    capitalExpenditures: Option<Value>,
    investments: Option<Value>,
    totalCashflowsFromInvestingActivities: Option<Value>,
    dividendsPaid: Option<Value>,
    netBorrowings: Option<Value>,
    otherCashflowsFromFinancingActivities: Option<Value>,
    totalCashFromFinancingActivities: Option<Value>,
    effectOfExchangeRate: Option<Value>,
    changeInCash: Option<Value>,
    repurchaseOfStock: Option<Value>,
    issuanceOfStock: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Value {
    raw: Option<i64>,
    fmt: Option<String>,
    longFmt: Option<String>,
}

fn make_url(ticker: &str) -> String {
    fmt::format(format_args!(
        r"https://query2.finance.yahoo.com/v10/finance/quoteSummary/{}.ME?formatted=true&crumb=th3yI76E9D%2F&lang=en-US&region=US&modules=incomeStatementHistory%2CcashflowStatementHistory%2CbalanceSheetHistory%2CincomeStatementHistoryQuarterly%2CcashflowStatementHistoryQuarterly%2CbalanceSheetHistoryQuarterly&corsDomain=finance.yahoo.com",
        ticker // MOEX // GAZP // SBER // YNDX // MTSS // ALRS // LKOH // ROSN // GMKN // AFLT
    ))
}

pub async fn make_request() -> Result<(), Box<dyn std::error::Error>> {
    let url = make_url("MOEX");
    let client = reqwest::Client::builder()
        .user_agent(CHROME_USER_AGENT)
        .build()?;
    let resp = client.get(&url).send().await?.json::<Response>().await?;
    println!("{:#?}", resp);
    Ok(())
}
