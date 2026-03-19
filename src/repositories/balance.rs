use std::sync::{Arc, RwLock};

pub trait BalanceRepository:  Send + Sync {
    fn get(&self) -> i32;
    fn set(&self, balance: i32);
}

pub struct InMemoryBalanceRepository {
    balance: RwLock<i32>,
}

impl InMemoryBalanceRepository {
    pub fn new(initial: i32) -> Self {
        Self{
            balance: RwLock::new(initial),
        }
    }
}

impl BalanceRepository for InMemoryBalanceRepository {
    fn get(&self) -> i32 {
        *self.balance.read().unwrap()
    }

    fn set(&self, balance: i32) {
        *self.balance.write().unwrap() = balance;
    }
}

pub type SharedBalanceRepository = Arc<dyn BalanceRepository>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_balance_repository() {
        let repo = InMemoryBalanceRepository::new(100);
        assert_eq!(repo.get(), 100);
        repo.set(150);
        assert_eq!(repo.get(), 150);
    }
    
    #[test]
    fn test_inmemory_balance_repository() {
        let repo = InMemoryBalanceRepository::new(100);
        assert_eq!(repo.get(), 100);
        repo.set(150);
        assert_eq!(repo.get(), 150);
    }
}