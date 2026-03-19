mod models;
mod services;
mod handlers;
mod repositories;
pub mod errors;

use std::net::SocketAddr;
use std::sync::{Arc};
use axum::{routing::get, routing::post, Router, http::{Request, Response}};
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tower_http::trace::TraceLayer;
use models::app_state::{AppState, SharedAppState};
use crate::repositories::balance::{InMemoryBalanceRepository};
use crate::services::balance::BalanceService;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    info!("init");
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
        .with_state(app_state)
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            tracing::info_span!("request", method = %request.method(), path = %request.uri().path())
        })
            .on_response(|_response: &Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                info!("response {} ms", latency.as_millis());
            }));

    info!("start server");
    axum::serve(listener, app).await.unwrap();
    info!("exit server");
}

