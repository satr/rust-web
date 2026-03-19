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