use crate::*;
use std::cell::Cell;

thread_local! {
    static SEED: Cell<Option<AlienSeed>> = Cell::new(None);
}

pub fn with_seed<F, R>(seed: AlienSeed, f: F) -> R
where
    F: FnOnce() -> R,
{
    SEED.with(|cell| {
        assert!(cell.get().is_none(), "Seed is already set");
        cell.set(Some(seed));
    });

    let result = f();

    SEED.with(|cell| cell.set(None));

    result
}

fn with_seed_replaced<F, R>(seed: AlienSeed, f: F) -> R
where
    F: FnOnce() -> R,
{
    let old = SEED.with(|cell| cell.replace(Some(seed)));

    let result = f();

    SEED.with(|cell| cell.set(old));

    result
}

pub fn with_random_batch<F, R>(n: usize, f: impl Fn() -> R) -> Vec<R> {
    let mut results = Vec::with_capacity(n);
    SEED.with(|cell| {
        let parent_seed = cell.get().expect("Parent seed must be set");
        for i in 0..n {
            let child_seed = parent_seed.child(i as u64);
            results.push(with_seed_replaced(child_seed, &f));
        }
    });
    results
}
