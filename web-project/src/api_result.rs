use std::time::Duration;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug)]
pub enum ApiResult <T> {
    Success(T),
    ApiError(String),
    NetworkError(String),
}

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}