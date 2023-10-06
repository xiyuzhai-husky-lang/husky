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
    send_task: JoinHandle<()>,
    send_channel: tokio::sync::mpsc::UnboundedSender<TraceClientSendMessage>,
    recv_task: JoinHandle<()>,
    recv_channel: tokio::sync::mpsc::UnboundedReceiver<TraceClientSendMessage>,
}

pub struct TraceClientSendMessage {}

impl TraceClientConnection {
    // ad hoc
    // todo: should use separate threads to create this
    // but let's ignore it for now
    #[tokio::main]
    pub async fn new(server: &str) -> TraceClientResult<Self> {
        todo!()
        // use futures_util::StreamExt;
        // let (stream, response) = connect_async(server).await?;
        // println!("Server response was {response:?}");
        // let (mut sender, mut receiver) = stream.split();
        // let mut send_task = tokio::spawn(async move {
        //     for i in 1..30 {
        //         // In any websocket error, break loop.
        //         if sender
        //             .send(Message::Text(format!("Message number {i}...")))
        //             .await
        //             .is_err()
        //         {
        //             //just as with server, if send fails there is nothing we can do but exit.
        //             return;
        //         }

        //         tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        //     }

        //     // When we are done we may want our client to close connection cleanly.
        //     println!("Sending close to {who}...");
        //     if let Err(e) = sender
        //         .send(Message::Close(Some(CloseFrame {
        //             code: CloseCode::Normal,
        //             reason: Cow::from("Goodbye"),
        //         })))
        //         .await
        //     {
        //         println!("Could not send Close due to {e:?}, probably it is ok?");
        //     };
        // });
        // Ok(Self {
        //     send_task: todo!(),
        //     recv_task: todo!(),
        // })
    }
}

impl Drop for TraceClientConnection {
    #[tokio::main]
    async fn drop(&mut self) {
        tokio::select! {
            _ = (&mut self.send_task) => {
                self.recv_task.abort();
            },
            _ = (&mut self.recv_task) => {
                self.send_task.abort();
            }
        }
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
