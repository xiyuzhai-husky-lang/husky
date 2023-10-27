use super::*;

const MOCK_SERVER: &str = "ws://localhost:51718/ws";

#[cfg(feature = "mock")]
impl TraceClient<()> {
    pub fn new_mock(tokio_runtime: Arc<tokio::runtime::Runtime>) -> Self {
        Self::new(tokio_runtime, MOCK_SERVER)
    }
}
