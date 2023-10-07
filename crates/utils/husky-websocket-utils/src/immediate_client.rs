use notify_change::NotifyEvent;
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::{
    self,
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

/// non-blocking
///
/// all apis are sync
pub struct ImmediateWebsocketClientConnection<Request, Response, ServerMessageArrivalNotifier>
where
    ServerMessageArrivalNotifier: NotifyEvent,
{
    server_address: String,
    create_task: JoinHandle<()>,
    status: WebsocketClientConnectionStatus<Request, Response, ServerMessageArrivalNotifier>,
}

pub enum WebsocketClientConnectionStatus<Request, Response, ServerMessageArrivalNotifier>
where
    ServerMessageArrivalNotifier: NotifyEvent,
{
    Await(Arc<Mutex<WebsocketClientConnectionAwaitStatus<Request, ServerMessageArrivalNotifier>>>),
    Ok {
        send_tx: tokio::sync::mpsc::UnboundedSender<Request>,
        recv_rx: tokio::sync::mpsc::UnboundedReceiver<Response>,
    },
    Err(WebsocketClientConnectionError),
}

pub enum WebsocketClientConnectionAwaitStatus<Request, ResponseNotifier>
where
    ResponseNotifier: NotifyEvent,
{
    Await,
    Ok {
        stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        response: tungstenite::handshake::client::Response,
        notifier: ResponseNotifier,
        init_request: Request,
    },
    Err(WebsocketClientConnectionError),
}

#[derive(Debug, Error)]
pub enum WebsocketClientConnectionError {}

impl<Request, Response, ResponseNotifier: NotifyEvent>
    ImmediateWebsocketClientConnection<Request, Response, ResponseNotifier>
where
    Request: Send + 'static,
{
    #[tokio::main]
    pub async fn new(
        server_address: String,
        init_request: Request,
        notifier: ResponseNotifier,
    ) -> Self {
        let await_status = Arc::new(Mutex::new(WebsocketClientConnectionAwaitStatus::Await));
        let create_task = tokio::spawn({
            let server_address = server_address.clone();
            let await_status = await_status.clone();
            async move {
                match tokio_tungstenite::connect_async(server_address).await {
                    Ok((stream, response)) => {
                        *await_status.lock().unwrap() = WebsocketClientConnectionAwaitStatus::Ok {
                            stream,
                            response,
                            notifier,
                            init_request,
                        }
                    }
                    Err(_) => todo!(),
                }
            }
        });
        Self {
            server_address,
            create_task,
            status: WebsocketClientConnectionStatus::Await(await_status),
        }
    }

    pub fn error(&self) -> Option<&WebsocketClientConnectionError> {
        match self.status {
            WebsocketClientConnectionStatus::Err(ref e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "serde_json")]
impl<ClientMessage, ServerMessage, ServerMessageArrivalNotifier>
    ImmediateWebsocketClientConnection<ClientMessage, ServerMessage, ServerMessageArrivalNotifier>
where
    ClientMessage: serde::Serialize + Send + Sync + 'static,
    ServerMessage: for<'a> serde::Deserialize<'a> + Send + Sync + 'static,
    ServerMessageArrivalNotifier: NotifyEvent,
{
    pub fn send(&mut self, msg: ClientMessage) {
        self.refresh();
        match self.status {
            WebsocketClientConnectionStatus::Await { .. } => todo!(),
            WebsocketClientConnectionStatus::Ok {
                ref mut send_tx, ..
            } => match send_tx.send(msg) {
                Ok(_) => (),
                Err(_) => todo!(),
            },
            WebsocketClientConnectionStatus::Err(_) => todo!(),
        }
    }

    pub fn try_recv(&mut self) -> Option<ServerMessage> {
        match self.status {
            WebsocketClientConnectionStatus::Await { .. } => todo!(),
            WebsocketClientConnectionStatus::Ok {
                ref mut recv_rx, ..
            } => match recv_rx.try_recv() {
                Ok(msg) => Some(msg),
                Err(e) => match e {
                    tokio::sync::mpsc::error::TryRecvError::Empty => None,
                    tokio::sync::mpsc::error::TryRecvError::Disconnected => todo!(),
                },
            },
            WebsocketClientConnectionStatus::Err(_) => todo!(),
        }
    }

    fn refresh(&mut self) -> StatusChanged {
        let await_result = match self.status {
            WebsocketClientConnectionStatus::Await(ref await_status) => match std::mem::replace(
                &mut *await_status.lock().unwrap(),
                WebsocketClientConnectionAwaitStatus::Await,
            ) {
                WebsocketClientConnectionAwaitStatus::Await => return StatusChanged::False,
                WebsocketClientConnectionAwaitStatus::Ok {
                    stream,
                    response,
                    init_request,
                    notifier,
                } => Ok((stream, response, init_request, notifier)),
                WebsocketClientConnectionAwaitStatus::Err(e) => Err(e),
            },
            WebsocketClientConnectionStatus::Ok { .. }
            | WebsocketClientConnectionStatus::Err(_) => return StatusChanged::False,
        };
        match await_result {
            Ok((stream, response, init_request, notifier)) => {
                self.status = Self::split_stream(stream, notifier)
            }
            Err(e) => self.status = WebsocketClientConnectionStatus::Err(e),
        }
        StatusChanged::True
    }

    fn split_stream(
        stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        notifier: ServerMessageArrivalNotifier,
    ) -> WebsocketClientConnectionStatus<ClientMessage, ServerMessage, ServerMessageArrivalNotifier>
    {
        use futures_util::{SinkExt, StreamExt};
        let (mut sender, mut receiver) = stream.split();
        let (send_tx, mut send_rx) = tokio::sync::mpsc::unbounded_channel::<ClientMessage>();
        let (mut recv_tx, recv_rx) = tokio::sync::mpsc::unbounded_channel::<ServerMessage>();
        let mut send_task = tokio::spawn(async move {
            while let Some(msg) = send_rx.recv().await {
                match serde_json::to_string(&msg) {
                    Ok(msg) => {
                        if let Err(_) = sender.send(Message::Text(msg)).await {
                            todo!()
                        }
                    }
                    Err(_) => todo!(),
                }
            }

            // When we are done we may want our client to close connection cleanly.
            let who = "who";
            println!("Sending close to {who}...");
            if let Err(e) = sender
                .send(Message::Close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: Cow::from("Goodbye"),
                })))
                .await
            {
                println!("Could not send Close due to {e:?}, probably it is ok?");
            };
        });
        let mut recv_task = tokio::spawn({
            async move {
                while let Some(msg) = receiver.next().await {
                    match msg {
                        Ok(msg) => match msg {
                            Message::Text(msg) => match serde_json::from_str(&msg) {
                                Ok(msg) => {
                                    if let Err(e) = recv_tx.send(msg) {
                                        todo!()
                                    } else {
                                        notifier.notify_event()
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
                        Err(_) => todo!(),
                    }
                }
                todo!()
            }
        });
        tokio::spawn(async move {
            tokio::select! {
                _ = &mut send_task => recv_task.abort(),
                _ = &mut recv_task => send_task.abort(),
            }
        });
        WebsocketClientConnectionStatus::Ok { send_tx, recv_rx }
    }
}

pub enum StatusChanged {
    True,
    False,
}
