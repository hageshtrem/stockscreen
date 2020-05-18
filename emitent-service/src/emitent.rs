use crate::report;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// pub trait Emitent {
//     fn new(id: u64, name: &str, description: &str) -> Self;
//     fn add_stock(&mut self, ticker: &str);
//     fn add_report(
//         &mut self,
//         date: Date<Utc>,
//         income: report::IncomeStatement,
//         balance: report::BalanceSheet,
//         cashflow: report::CashFlow,
//     );
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Security {
    ticker: String,
    r#type: SecurityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SecurityType {
    Bond,
    Stock,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emitent {
    #[serde(with = "bson::compat::u2f")]
    pub id: u64,
    pub name: String,
    pub description: String,
    stocks: Vec<Security>,
    reports: Vec<report::Report>,
}

impl Emitent {
    pub fn new(id: u64, name: &str, description: &str) -> Self {
        Emitent {
            id,
            name: name.to_owned(),
            description: description.to_owned(),
            stocks: vec![],
            reports: vec![],
        }
    }

    fn add_stock(&mut self, ticker: &str) {
        self.stocks.push(Security {
            ticker: ticker.to_owned(),
            r#type: SecurityType::Stock,
        })
    }

    fn add_report(
        &mut self,
        date: DateTime<Utc>,
        income: report::IncomeStatement,
        balance: report::BalanceSheet,
        cashflow: report::CashFlow,
    ) {
        self.reports
            .push(report::Report::new(date, income, balance, cashflow))
    }
}

#[async_trait]
pub trait EmitentRepository: Send + Sync + 'static {
    fn store(&self, e: &Emitent) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_all(&self) -> Result<Vec<Emitent>, Box<dyn std::error::Error>>;
}

// struct EmitentService {
//     rerository: dyn EmitentRepository,
// }

// impl EmitentService {
//     fn new_emitent(&self, name: &str, description: &str) -> Result<(), Box<dyn std::error::Error>> {
//         let e = Emitent::new(17, name, description);
//         self.rerository.store(&e)?;
//         Ok(())
//     }
// }
