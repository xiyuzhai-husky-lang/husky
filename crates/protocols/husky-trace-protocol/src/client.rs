pub mod error;
pub mod mock;

use self::error::*;
use crate::*;
use husky_visual_protocol::mock::MockVisualProtocol;
#[cfg(feature = "mock")]
use husky_visual_protocol::IsVisualProtocol;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;

pub struct TraceClientConnection {
    ws_sender: (),
    ws_recv_task: JoinHandle<()>,
}

impl TraceClientConnection {
    #[tokio::main]
    pub async fn new(server: &str) -> TraceClientResult<Self> {
        let (stream, _response) = connect_async(server).await?;
        Ok(Self {
            ws_sender: todo!(),
            ws_recv_task: todo!(),
        })
    }
}

pub struct TraceClient<VisualProtocol: IsVisualProtocol> {
    cache: Option<TraceCache<VisualProtocol>>,
    connection_result: TraceClientResult<TraceClientConnection>,
}

impl<VisualProtocol: IsVisualProtocol> TraceClient<VisualProtocol> {
    pub fn new(server: &str) -> Self {
        Self {
            cache: None,
            connection_result: TraceClientConnection::new(server),
        }
    }

    pub fn root_trace_ids(&self) -> Option<TraceIdRange> {
        Some(self.cache.as_ref()?.root_trace_ids())
    }

    pub fn connection_result(&self) -> Result<&TraceClientConnection, &TraceClientError> {
        self.connection_result.as_ref()
    }
}

impl<VisualProtocol: IsVisualProtocol> std::ops::Index<TraceIdRange>
    for TraceClient<VisualProtocol>
{
    type Output = [TraceCacheEntry];

    fn index(&self, trace_id_range: TraceIdRange) -> &Self::Output {
        &self.cache.as_ref().unwrap()[trace_id_range]
    }
}
