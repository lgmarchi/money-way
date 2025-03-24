use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: u64,
    pub user_id: u64,
    pub category_id: u64,
    pub r#type: String,
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateTransactionRequest {
    pub category_id: u64,
    pub r#type: String,
    pub amount: u64,
    pub memo: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTransactionRequest {
    pub memo: String,
    pub description: Option<String>,
}
