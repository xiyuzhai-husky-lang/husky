use crate::*;
use std::cell::Cell;
use tokio::task_local;

task_local! {
    static SEED: AlienSeed;
}

pub fn with_seed<R>(seed: AlienSeed, f: impl FnOnce() -> R) -> R {
    assert!(SEED.try_with(|_| ()).is_err(), "Seed is already set");
    SEED.sync_scope(seed, f)
}

impl AlienSeed {
    pub fn with<R>(self, f: impl FnOnce() -> R) -> R {
        with_seed(self, f)
    }
}

#[track_caller]
pub fn attached_seed() -> AlienSeed {
    SEED.try_with(|&seed| seed).expect("Seed not set.")
}

fn with_seed_replaced<R>(seed: AlienSeed, f: impl FnOnce() -> R) -> R {
    SEED.sync_scope(seed, f)
}

pub fn with_random_batch<R>(n: usize, f: impl Fn() -> R) -> Vec<R> {
    let mut results = Vec::with_capacity(n);
    SEED.with(|parent_seed| {
        for i in 0..n {
            let child_seed = parent_seed.child(i as u64);
            results.push(with_seed_replaced(child_seed, &f));
        }
    });
    results
}
