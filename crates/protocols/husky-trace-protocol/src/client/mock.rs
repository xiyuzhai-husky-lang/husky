use notify::Notify;

use super::*;

const MOCK_SERVER: &str = "ws://localhost:51718/ws";

#[cfg(feature = "mock")]
impl<Notifier> TraceClient<(), Notifier>
where
    Notifier: Notify,
{
    pub fn new_mock(tokio_runtime: Arc<tokio::runtime::Runtime>, notifier: Notifier) -> Self {
        Self::new(tokio_runtime, MOCK_SERVER, notifier)
    }
}
