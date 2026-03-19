use crate::services::balance::SharedBalanceService;
use std::sync::Arc;

pub struct AppState {
    pub balance_service: SharedBalanceService,
}

pub type SharedAppState = Arc<AppState>;