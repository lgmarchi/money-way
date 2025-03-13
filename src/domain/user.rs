use serde::Deserialize;

pub struct User {}

#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
