use super::*;

const MOCK_SERVER: &str = "ws://127.0.0.1:51718/ws";

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
