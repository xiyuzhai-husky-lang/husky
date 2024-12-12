use chrono::Utc;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmCacheEntry<Request, Response> {
    pub(crate) request: Request,
    pub(crate) response: Response,
    pub(crate) time: DateTime<Utc>,
}

impl<Request, Response> LlmCacheEntry<Request, Response> {
    /// Create a new cache entry with the current timestamp
    pub fn new(request: Request, response: Response) -> Self {
        Self {
            request,
            response,
            time: Utc::now(),
        }
    }
}
