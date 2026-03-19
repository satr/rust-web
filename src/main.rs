mod models;
mod services;
mod handlers;
mod repositories;
pub mod errors;

use std::net::SocketAddr;
use std::sync::{Arc};
use axum::{routing::get, routing::post, Router};
use tokio::net::TcpListener;
use tracing::trace;
use models::app_state::{AppState, SharedAppState};
use crate::repositories::balance::{InMemoryBalanceRepository};
use crate::services::balance::BalanceService;

#[tokio::main]
async fn main() {
    trace!("init");
    let addr = SocketAddr::from(([127,0,0,1], 3001));
    let listener = TcpListener::bind(addr).await.unwrap();
    let balance_repo = Arc::new(InMemoryBalanceRepository::new(100));
    let balance_service = Arc::new(BalanceService::new(balance_repo));
    let app_state: SharedAppState = Arc::new(
        AppState{
            balance_service
        }
    );
    let app = Router::new()
        .route("/health", get(|| async {"Ok"}))
        .route("/balance", get(handlers::balance::get_balance))
        .route("/deposit", post(handlers::balance::deposit))
        .with_state(app_state);
    trace!("start server");
    axum::serve(listener, app).await.unwrap();
    trace!("exit server");
}

