use crate::helpers::serde_helpers::deserialize_null_default;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateUser {
    #[serde(deserialize_with = "deserialize_null_default")]
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    #[validate(length(
        min = 5,
        max = 50,
        message = "Name must be between 5 and 50 characters"
    ))]
    pub name: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    #[validate(length(
        min = 5,
        max = 15,
        message = "Password must be between 5 and 15 characters"
    ))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginUser {
    #[serde(deserialize_with = "deserialize_null_default")]
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    #[validate(length(
        min = 5,
        max = 15,
        message = "Password must be between 5 and 15 characters"
    ))]
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub email: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct FilterUser {
    #[serde(deserialize_with = "deserialize_null_default")]
    pub id: Option<i32>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub email: Option<String>,
}
