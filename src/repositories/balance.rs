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
    fn init_balance() {
        let repo1 = InMemoryBalanceRepository::new(100);
        let repo2 = InMemoryBalanceRepository::new(200);
        assert_eq!(repo1.get(), 100);
        assert_eq!(repo2.get(), 200);
    }

    #[test]
    fn set_balance() {
        let repo1 = InMemoryBalanceRepository::new(100);
        let repo2 = InMemoryBalanceRepository::new(200);

        repo1.set(150);

        assert_eq!(repo1.get(), 150);
        assert_eq!(repo2.get(), 200);
    }
}