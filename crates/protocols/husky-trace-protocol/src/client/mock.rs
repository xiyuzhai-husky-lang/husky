use super::*;

const MOCK_SERVER: &str = "ws://127.0.0.1:3000/ws";

#[cfg(feature = "mock")]
impl TraceClient<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        let mut slf = Self::new(MOCK_SERVER);
        slf.cache = Some(TraceCache::new_mock());
        slf
    }
}
