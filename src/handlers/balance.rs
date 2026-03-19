use crate::errors::AppError;
use crate::models::app_state::SharedAppState;
use crate::models::requests::DepositRequest;
use crate::models::responds::BalanceResponse;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;

//GET /balance
pub async fn get_balance(State(app_state): State<SharedAppState>) -> Response {
    let balance = app_state.balance_service.get_balance();
    Json(BalanceResponse{ balance }).into_response()
}

//POST /deposit
pub async fn deposit(State(app_state): State<SharedAppState>, Json(payload): Json<DepositRequest>) -> Result<Json<BalanceResponse>, AppError> {
    let balance = app_state.balance_service.deposit(payload.amount)?;
    Ok(Json(BalanceResponse{balance}))
}