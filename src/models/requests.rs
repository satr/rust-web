use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DepositRequest {
    pub amount: i32,
}