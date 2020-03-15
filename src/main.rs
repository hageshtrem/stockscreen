use serde::{Deserialize, Serialize};

static CHROME_USER_AGENT: &str = r"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.87 Safari/537.36";

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
    netReceivables: Value,
    otherCurrentAssets: Value,
    totalCurrentAssets: Value,
    propertyPlantEquipment: Value,
    goodWill: Value,
    intangibleAssets: Value,
    otherAssets: Value,
    deferredLongTermAssetCharges: Value,
    totalAssets: Value,
    accountsPayable: Value,
    otherCurrentLiab: Value,
    otherLiab: Value,
    minorityInterest: Value,
    totalCurrentLiabilities: Value,
    totalLiab: Value,
    commonStock: Value,
    retainedEarnings: Value,
    treasuryStock: Value,
    capitalSurplus: Value,
    otherStockholderEquity: Value,
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
    depreciation: Value,
    changeToNetincome: Value,
    changeToLiabilities: Value,
    changeToOperatingActivities: Value,
    totalCashFromOperatingActivities: Value,
    capitalExpenditures: Value,
    investments: Value,
    totalCashflowsFromInvestingActivities: Value,
    dividendsPaid: Option<Value>,
    netBorrowings: Value,
    totalCashFromFinancingActivities: Value,
    effectOfExchangeRate: Value,
    changeInCash: Value,
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
    let client = reqwest::Client::builder()
        .user_agent(CHROME_USER_AGENT)
        .build()?;
    let resp = client.get(URL).send().await?.json::<Response>().await?;
    println!("{:#?}", resp);
    Ok(())
}
