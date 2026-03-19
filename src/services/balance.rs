use std::sync::Arc;
use crate::errors::AppError;
use crate::repositories::balance::SharedBalanceRepository;

pub struct BalanceService {
    repo: SharedBalanceRepository,
}

pub type SharedBalanceService = Arc<BalanceService>;

impl BalanceService {
    pub fn new(repo: SharedBalanceRepository) -> Self {
        Self{repo}
    }

    pub fn get_balance(&self) -> i32 {
        self.repo.get()
    }

    pub fn deposit(&self, amount: i32) -> Result<i32, AppError> {
        if amount <= 0 {
            return Err(AppError::InvalidAmount);
        }
        let balance = self.repo.get();
        match balance.checked_add(amount) {
            Some(new_balance) => {
                self.repo.set(new_balance);
                Ok(new_balance)
            }
            None => Err(AppError::BalanceOverflow),
        }
    }
}