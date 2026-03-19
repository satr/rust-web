#[derive(Debug, PartialEq, Eq)]
pub enum AppError{
    InvalidAmount,
    BalanceOverflow,
}