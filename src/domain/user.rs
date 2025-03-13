use std::time::SystemTime;

use actix_web::rt::System;
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

        Self {
            sub,
            role,
            exp: exp.unwrap_or_else(|| four_hours_ahead_since_epoch),
        }
    }
}
