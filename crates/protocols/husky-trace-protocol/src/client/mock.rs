use super::*;

const MOCK_SERVER: &str = "localhost:51718";

#[cfg(feature = "mock")]
impl TraceClient<MockVisualProtocol, ()> {
    pub fn new_mock() -> Self {
        let mut slf = Self::new(MOCK_SERVER, ());
        if slf.root_trace_ids().is_none() {
            slf.cache = TraceCache::new_mock();
        }
        slf
    }
}
