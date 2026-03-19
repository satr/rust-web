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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::repositories::balance::BalanceRepository;
    use super::*;
    struct FakeBalanceRepository {
        balance: Mutex<i32>,
    }

    impl FakeBalanceRepository {
        fn new(initial: i32) -> Self {
            Self {
                balance: Mutex::new(initial),
            }
        }
    }

    impl BalanceRepository for FakeBalanceRepository {
        fn get(&self) -> i32 {
            *self.balance.lock().unwrap()
        }
        fn set(&self, balance: i32) {
            *self.balance.lock().unwrap() = balance;
        }
    }

    #[test]
    fn get_balance_returns_current_balance() {
        let repo: SharedBalanceRepository = Arc::new(FakeBalanceRepository::new(100));
        let service = BalanceService::new(repo);

        let balance = service.get_balance();

        assert_eq!(balance, 100);
    }

    #[test]
    fn deposit_returns_new_balance() {
        let repo: SharedBalanceRepository = Arc::new(FakeBalanceRepository::new(100));
        let service = BalanceService::new(repo);

        let balance_result = service.deposit(50);

        assert_eq!(balance_result.unwrap(), 150);
    }

    #[test]
    fn deposit_rejects_negative_balance() {
        let repo: SharedBalanceRepository = Arc::new(FakeBalanceRepository::new(100));
        let service = BalanceService::new(repo);

        let balance_result = service.deposit(-10);

        assert!(matches!(balance_result, Err(AppError::InvalidAmount)));
    }

    #[test]
    fn deposit_rejects_zero_balance() {
        let repo: SharedBalanceRepository = Arc::new(FakeBalanceRepository::new(100));
        let service = BalanceService::new(repo);

        let balance_result = service.deposit(0);

        assert!(matches!(balance_result, Err(AppError::InvalidAmount)));
    }

    #[test]
    fn deposit_fails_on_overflow_deposit() {
        let repo: SharedBalanceRepository = Arc::new(FakeBalanceRepository::new(i32::MAX));
        let service = BalanceService::new(repo);

        let balance_result = service.deposit(1);

        assert!(matches!(balance_result, Err(AppError::BalanceOverflow)));
    }
}