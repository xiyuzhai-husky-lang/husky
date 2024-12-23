use chrono::Utc;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmCacheEntry<Seed, Request, Response> {
    pub(crate) seed: Seed,
    pub(crate) request: Request,
    pub(crate) response: Response,
    pub(crate) time: DateTime<Utc>,
}

impl<Seed, Request, Response> LlmCacheEntry<Seed, Request, Response> {
    /// Create a new cache entry with the current timestamp
    pub fn new(seed: Seed, request: Request, response: Response) -> Self {
        Self {
            seed,
            request,
            response,
            time: Utc::now(),
        }
    }
}
