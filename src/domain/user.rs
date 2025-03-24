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

#[derive(Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(default)]
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
    /// Creates a new Claims instance with the given subject ID and role.
    /// Optionally accepts an expiration timestamp, defaulting to 4 hours from
    /// now if not provided.
    ///
    /// # Arguments
    /// * `sub` - The subject (user) ID
    /// * `role` - The role of the user
    /// * `exp` - Optional expiration timestamp in seconds since Unix epoch
    pub fn new(sub: u64, role: String, exp: Option<u64>) -> Self {
        const FOUR_HOURS_IN_SECS: u64 = 4 * 60 * 60;

        let default_expiration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time should be after Unix epoch")
            .as_secs()
            + FOUR_HOURS_IN_SECS;

        Self { sub, role, exp: exp.unwrap_or(default_expiration) }
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub first_name: String,
    pub last_name: String,
}
