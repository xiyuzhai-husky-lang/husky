use super::*;
use crate::protocol::mock::MockTraceProtocol;
use notify_change::NotifyChange;

const MOCK_SERVER: &str = "ws://localhost:51718/ws";

#[cfg(feature = "mock")]
impl<TraceProtocol: IsTraceProtocolFull, Notifier> TraceClient<TraceProtocol, Notifier>
where
    Notifier: NotifyChange,
{
    pub fn new_mock(tokio_runtime: Arc<tokio::runtime::Runtime>, notifier: Notifier) -> Self {
        Self::new(tokio_runtime, MOCK_SERVER, notifier)
    }
}
