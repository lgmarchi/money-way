use std::time::SystemTime;

use serde::{
    Deserialize,
    Serialize,
};
use sqlx::types::chrono::{
    DateTime,
    Utc,
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

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub balance: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: u64,
    pub role: String,
    pub exp: u64,
}

impl Claims {
    pub fn new(sub: u64, role: String, exp: Option<u64>) -> Self {
        let four_hours_ahead_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60;

        Self { sub, role, exp: exp.unwrap_or(four_hours_ahead_since_epoch) }
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub first_name: String,
    pub last_name: String,
}
