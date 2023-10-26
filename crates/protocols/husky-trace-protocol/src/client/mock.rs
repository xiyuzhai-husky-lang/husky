use super::*;

const MOCK_SERVER: &str = "ws://127.0.0.1:51718/ws";

#[cfg(feature = "mock")]
impl TraceClient<()> {
    pub fn new_mock(tokio_runtime: Arc<tokio::runtime::Runtime>) -> Self {
        let mut slf = Self::new(tokio_runtime, MOCK_SERVER);
        if slf.root_trace_ids().is_none() {
            slf.cache = Some(TraceCache::new_mock());
        }
        slf
    }
}
