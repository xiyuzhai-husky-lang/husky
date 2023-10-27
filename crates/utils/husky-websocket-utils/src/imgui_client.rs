//! meant for immediate mode gui
use futures_util::{SinkExt, StreamExt};
use husky_print_utils::p;
use std::{borrow::Cow, sync::Arc};
use thiserror::Error;
use tokio_tungstenite::tungstenite::{
    self,
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

const ORDERING: core::sync::atomic::Ordering = core::sync::atomic::Ordering::SeqCst;

/// non-blocking
///
/// all apis are sync
pub struct ImmediateWebsocketClientConnection<Request, Response> {
    tokio_runtime: Arc<tokio::runtime::Runtime>,
    server_address: String,
    connect_join_handle: tokio::task::JoinHandle<()>,
    creation_status: CreationStatus<Request, Response>,
    request_tx: tokio::sync::mpsc::Sender<Request>,
    response_rx: tokio::sync::mpsc::Receiver<Response>,
    communication_status: Arc<AtomicCommunicationStatus>,
    /// must use std JoinHandle
    launch_join_handle: Option<tokio::task::JoinHandle<()>>,
}

#[atomic_enum::atomic_enum]
pub enum CommunicationStatus {
    Creation,
    AwaitingRequest,
    DeserializingRequest,
    AwaitingResponse,
    SerializingResponse,
    ResponseReady,
}

pub enum CreationStatus<Request, Response> {
    Await(Arc<std::sync::Mutex<CreationAwaitStatus<Request, Response>>>),
    Ok,
    Err(WebsocketClientConnectionError),
}

pub enum CreationAwaitStatus<Request, Response> {
    Await,
    Ok {
        stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        response: tungstenite::handshake::client::Response,
        request_rx: tokio::sync::mpsc::Receiver<Request>,
        response_tx: tokio::sync::mpsc::Sender<Response>,
    },
    Err(WebsocketClientConnectionError),
}

#[derive(Debug, Error)]
pub enum WebsocketClientConnectionError {
    #[error("send request while creation")]
    SendRequestWhileCreation,
    #[error("send request while response not processed")]
    SendRequestWhileResponseNotProcessed,
}

impl<Request, Response> ImmediateWebsocketClientConnection<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    pub fn new(tokio_runtime: Arc<tokio::runtime::Runtime>, server_address: String) -> Self {
        let await_status = Arc::new(std::sync::Mutex::new(CreationAwaitStatus::Await));
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(1);
        let (response_tx, response_rx) = tokio::sync::mpsc::channel(1);
        let connect_join_handle = {
            let server_address = server_address.clone();
            let await_status = await_status.clone();
            tokio_runtime.spawn(async move {
                println!("server_address = {server_address}");
                match tokio_tungstenite::connect_async(server_address).await {
                    Ok((stream, response)) => {
                        *await_status.lock().unwrap() = CreationAwaitStatus::Ok {
                            stream,
                            response,
                            request_rx,
                            response_tx,
                        }
                    }
                    Err(e) => {
                        p!(e);
                        todo!()
                    }
                }
            })
        };
        Self {
            server_address,
            tokio_runtime,
            connect_join_handle,
            creation_status: CreationStatus::Await(await_status),
            request_tx,
            response_rx,
            communication_status: Arc::new(AtomicCommunicationStatus::new(
                CommunicationStatus::Creation,
            )),
            launch_join_handle: None,
        }
    }

    pub fn error(&self) -> Option<&WebsocketClientConnectionError> {
        match self.creation_status {
            CreationStatus::Err(ref e) => Some(e),
            _ => None,
        }
    }
}

pub trait NeedResponse {
    fn need_response(&self) -> bool;
}

#[cfg(feature = "serde_json")]
impl<Request, Response> ImmediateWebsocketClientConnection<Request, Response>
where
    Request: serde::Serialize + Send + 'static + NeedResponse + Default,
    Response: for<'a> serde::Deserialize<'a> + Send + 'static,
{
    pub fn try_send_request(
        &mut self,
        request: Request,
    ) -> Result<(), WebsocketClientConnectionError> {
        self.refresh();
        match self.communication_status.load(ORDERING) {
            CommunicationStatus::Creation => {
                Err(WebsocketClientConnectionError::SendRequestWhileCreation)
            }
            CommunicationStatus::AwaitingRequest => {
                self.request_tx.blocking_send(request).map_err(|e| todo!())
            }
            CommunicationStatus::DeserializingRequest
            | CommunicationStatus::AwaitingResponse
            | CommunicationStatus::SerializingResponse
            | CommunicationStatus::ResponseReady => {
                Err(WebsocketClientConnectionError::SendRequestWhileResponseNotProcessed)
            }
        }
    }

    pub fn try_recv(&mut self) -> Option<Response> {
        self.refresh();
        match self.communication_status.load(ORDERING) {
            CommunicationStatus::Creation
            | CommunicationStatus::AwaitingRequest
            | CommunicationStatus::DeserializingRequest
            | CommunicationStatus::AwaitingResponse
            | CommunicationStatus::SerializingResponse => None,
            CommunicationStatus::ResponseReady => {
                let response = self.response_rx.blocking_recv();
                self.communication_status
                    .store(CommunicationStatus::AwaitingRequest, ORDERING);
                response
            }
        }
    }

    fn refresh(&mut self) -> StatusChanged {
        let await_result = match self.creation_status {
            CreationStatus::Await(ref await_status) => match std::mem::replace(
                &mut *await_status.lock().unwrap(),
                CreationAwaitStatus::Await,
            ) {
                CreationAwaitStatus::Await => return StatusChanged::False,
                CreationAwaitStatus::Ok {
                    stream,
                    response,
                    request_rx,
                    response_tx,
                } => Ok((stream, response, request_rx, response_tx)),
                CreationAwaitStatus::Err(e) => Err(e),
            },
            CreationStatus::Ok { .. } | CreationStatus::Err(_) => return StatusChanged::False,
        };
        match await_result {
            Ok((stream, response, request_rx, response_tx)) => self.launch(
                stream,
                request_rx,
                response_tx,
                self.communication_status.clone(),
            ),
            Err(e) => self.creation_status = CreationStatus::Err(e),
        }
        StatusChanged::True
    }

    fn launch(
        &mut self,
        mut stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        mut request_rx: tokio::sync::mpsc::Receiver<Request>,
        response_tx: tokio::sync::mpsc::Sender<Response>,
        communication_status: Arc<AtomicCommunicationStatus>,
    ) {
        debug_assert!(self.launch_join_handle.is_none());
        self.launch_join_handle = Some(self.tokio_runtime.spawn(async move {
            communication_status.store(CommunicationStatus::AwaitingRequest, ORDERING);
            while let Some(request) = request_rx.recv().await {
                communication_status.store(CommunicationStatus::DeserializingRequest, ORDERING);
                match serde_json::to_string(&request) {
                    Ok(request) => {
                        if let Err(_) = stream.send(Message::Text(request)).await {
                            todo!()
                        }
                    }
                    Err(_) => todo!(),
                }
                if request.need_response() {
                    communication_status.store(CommunicationStatus::AwaitingResponse, ORDERING);
                    match stream.next().await {
                        Some(response) => match response {
                            Ok(response) => match response {
                                Message::Text(response) => match serde_json::from_str(&response) {
                                    Ok(response) => {
                                        if let Err(_) = response_tx.send(response).await {
                                            todo!()
                                        }
                                    }
                                    Err(_) => todo!(),
                                },
                                Message::Binary(_) => todo!(),
                                Message::Ping(_) => todo!(),
                                Message::Pong(_) => todo!(),
                                Message::Close(_) => todo!(),
                                Message::Frame(_) => todo!(),
                            },
                            Err(e) => todo!("e = {e}"),
                        },
                        None => todo!(),
                    }
                    communication_status.store(CommunicationStatus::ResponseReady, ORDERING);
                }
            }
        }));
        match self.request_tx.blocking_send(Request::default()) {
            Ok(_) => (),
            Err(e) => todo!("e = {e}"),
        };
        self.creation_status = CreationStatus::Ok
    }

    async fn launch_aux(
        mut stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        mut request_rx: tokio::sync::mpsc::Receiver<Request>,
        response_tx: tokio::sync::mpsc::Sender<Response>,
        communication_status: Arc<AtomicCommunicationStatus>,
    ) {

        // When we are done we may want our client to close connection cleanly.
        // let who = "who";
        // println!("Sending close to {who}...");
        // if let Err(e) = sender
        //     .send(Message::Close(Some(CloseFrame {
        //         code: CloseCode::Normal,
        //         reason: Cow::from("Goodbye"),
        //     })))
        //     .await
        // {
        //     println!("Could not send Close due to {e:?}, probably it is ok?");
        // };
    }
}

pub enum StatusChanged {
    True,
    False,
}
