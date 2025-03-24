use std::fmt;

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl TransactionType {
    pub fn is_credit(&self) -> bool {
        *self == TransactionType::Credit
    }

    pub fn is_debit(&self) -> bool {
        *self == TransactionType::Debit
    }
}

impl From<String> for TransactionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Credit" | "CREDIT" => TransactionType::Credit,
            "Debit" | "DEBIT" => TransactionType::Debit,
            _ => panic!("Invalid transaction type: {}", value),
        }
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TransactionType::Credit => "CREDIT",
            TransactionType::Debit => "DEBIT",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Transaction {
    pub id: u64,
    pub user_id: u64,
    pub category_id: u64,
    pub r#type: TransactionType,
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateTransactionRequest {
    pub category_id: u64,
    pub r#type: TransactionType,
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTransactionRequest {
    pub memo: String,
    pub description: Option<String>,
}
