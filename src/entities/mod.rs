use serde::Serialize;

pub mod cat;
pub mod user;

#[derive(Serialize)]
pub struct ResponseWrapper<T> {
    pub message: String,
    pub data: Option<T>,
}
