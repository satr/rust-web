use crate::errors::AppError;
use crate::models::app_state::SharedAppState;
use crate::models::requests::DepositRequest;
use crate::models::responds::BalanceResponse;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[cfg(test)]
use http_body_util::BodyExt;

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    use axum::{body::Body, http::{request::Builder, Method, Request, StatusCode}, Router};
    use serde_json::Value;
    use tower::ServiceExt;
    use crate::models::app_state::AppState;
    use crate::repositories::balance::{InMemoryBalanceRepository, SharedBalanceRepository};
    use crate::services::balance::BalanceService;

    #[tokio::test]
    async fn get_balance_returns_current_balance() {
        let response = make_app_request(|builder| {
            builder
                .uri("/balance")
                .header("content-type", "application/json")
                .method(Method::GET)
                .body(Body::empty())
        }, 100).await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn deposit_returns_new_balance() {
        let response = make_app_request(|builder| {
            builder
                .uri("/deposit")
                .method(Method::POST)
                .body(Body::from(r#"{"amount": 50}"#))
        }, 100).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body_json = get_body_json_from(response).await;
        assert_eq!(body_json["balance"], 150);
    }

    #[tokio::test]
    async fn deposit_rejects_negative_amount() {
        let response = make_app_request(|builder| {
            builder
                .uri("/deposit")
                .method(Method::POST)
                .body(Body::from(r#"{"amount": -5}"#))
        }, 100).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body_json = get_body_json_from(response).await;
        assert_eq!(body_json["error"], "Invalid amount");
    }

    #[tokio::test]
    async fn deposit_rejects_zero_amount() {
        let response = make_app_request(|builder| {
            builder
                .uri("/deposit")
                .method(Method::POST)
                .body(Body::from(r#"{"amount": 0}"#))
        }, 100).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body_json = get_body_json_from(response).await;
        assert_eq!(body_json["error"], "Invalid amount", "Invalid or missing error message");
    }

    #[tokio::test]
    async fn deposit_fails_on_overflow_deposit() {
        let response = make_app_request(|builder| {
            builder
                .uri("/deposit")
                .method(Method::POST)
                .body(Body::from(r#"{"amount": 1}"#))
        }, i32::MAX).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body_json = get_body_json_from(response).await;
        assert_eq!(body_json["error"], "Balance overflow", "Invalid or missing error message");
    }

    async fn get_body_json_from(response: Response) -> Value {
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
        body_json
    }

    async fn make_app_request<F>(modifier: F, initial: i32) -> Response
    where
        F: FnOnce(Builder) -> Result<Request<Body>, axum::http::Error>,
    {
        let request = modifier(
            Request::builder()
                .header("content-type", "application/json"),
        )
        .unwrap();

        let app = build_test_app(initial);
        app.oneshot(request).await.unwrap()
    }

    fn build_test_app(initial: i32) -> Router {
        let repo: SharedBalanceRepository = Arc::new(InMemoryBalanceRepository::new(initial));
        let shared_service = Arc::new(BalanceService::new(repo));
        let app_state = Arc::new(AppState{balance_service: shared_service });
        Router::new()
            .route("/balance", axum::routing::get(get_balance))
            .route("/deposit", axum::routing::post(deposit))
            .with_state(app_state)
    }
}