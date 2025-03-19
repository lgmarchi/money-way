use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
