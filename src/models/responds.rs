use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::errors::AppError;

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub balance: i32,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BalanceOverflow => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse{error: "Balance overflow".to_string()})
                    ).into_response(),
            AppError::InvalidAmount => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse{error: "Invalid amount".to_string()})
                ).into_response(),
        }
    }
}