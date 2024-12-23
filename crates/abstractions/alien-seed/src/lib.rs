pub mod attach;

use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlienSeed(NonZeroU64);

impl AlienSeed {
    pub fn new(seed: u64) -> Self {
        let mut x = seed;
        x = x ^ (x >> 30);
        x = x.wrapping_mul(0xbf58476d1ce4e5b9);
        x = x ^ (x >> 27);
        x = x.wrapping_mul(0x94d049bb133111eb);
        x = x ^ (x >> 31);

        Self(NonZeroU64::new(x.wrapping_add(1)).unwrap())
    }
}

impl AlienSeed {
    /// Creates a new seed by combining this seed with an index.
    /// This is useful for creating deterministic but different seeds
    /// for child processes or sub-sequences.
    ///
    /// # Example
    /// ```
    /// use alien_seed::AlienSeed;
    ///
    /// let parent = AlienSeed::from(1u64);
    /// let child1 = parent.child(0);
    /// let child2 = parent.child(1);
    /// assert_ne!(child1, child2); // Different indices produce different seeds
    /// assert_eq!(child1, parent.child(0)); // Same parent and index produces same seed
    /// ```
    pub fn child(&self, index: u64) -> Self {
        // Combine the current seed with the index using wrapping operations
        let seed = self.0.get().wrapping_mul(0xbf58476d1ce4e5b9);
        let combined = seed.wrapping_add(index);
        AlienSeed::from(combined)
    }
}

impl From<u64> for AlienSeed {
    fn from(seed: u64) -> Self {
        AlienSeed::new(seed)
    }
}
