mod permutation;

pub use permutation::*;

use std::time::{SystemTime, UNIX_EPOCH};

/// a wrapper for random generation
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

pub struct XRng {
    rng: StdRng,
}

impl XRng {
    pub fn new(seed: u64) -> Self {
        XRng {
            rng: StdRng::seed_from_u64(seed),
        }
    }
    pub fn new_time_seeded() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        XRng {
            rng: StdRng::seed_from_u64(since_the_epoch.as_millis() as u64),
        }
    }

    pub fn randint<T>(&mut self, a: T, b: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd,
    {
        self.rng.gen_range(a..b)
    }

    pub fn randidx(&mut self) -> usize {
        self.rng.next_u64() as usize
    }

    pub fn with_probability(&mut self, p: f32) -> bool {
        self.rng.gen_bool(p as f64)
    }

    pub fn randbool(&mut self) -> bool {
        self.rng.gen_bool(0.5)
    }
}

#[test]
fn haha() {
    let mut xrng = XRng::new(1213);
    println!("random number between 0 and 1: {}", xrng.randint(0, 2));
}
