use serde::{
    Deserialize,
    Serialize,
};
use sqlx::{
    FromRow,
    types::chrono::{
        self,
        DateTime,
        NaiveDateTime,
        Utc,
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub balance: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
