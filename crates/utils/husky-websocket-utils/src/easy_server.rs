use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::sync::Mutex;

pub trait IsEasyWebsocketServerState: Send + Sync + 'static {
    /// goes from server to client
    type Response: serde::Serialize + Send + 'static;
    /// goes from client to server
    type Request: for<'a> serde::Deserialize<'a> + Send + 'static;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response>;

    fn easy_server(self, addr: impl Into<SocketAddr>)
    where
        Self: Sized,
    {
        easy_server(Arc::new(Mutex::new(self)), addr)
    }
}

#[tokio::main]
pub async fn easy_server<S>(slf: std::sync::Arc<Mutex<S>>, addr: impl Into<SocketAddr>)
where
    S: IsEasyWebsocketServerState,
{
    easy_server_aux(slf, addr).await
}

async fn easy_server_aux<S>(slf: std::sync::Arc<Mutex<S>>, addr: impl Into<SocketAddr>)
where
    S: IsEasyWebsocketServerState,
{
    let router = Router::new()
        .route("/websocket", get(websocket_handler))
        .with_state(slf);
    let addr = addr.into();
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn websocket_handler<S>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<S>>>,
) -> impl IntoResponse
where
    S: IsEasyWebsocketServerState,
{
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket<S>(mut stream: WebSocket, state: Arc<Mutex<S>>)
where
    S: IsEasyWebsocketServerState,
{
    while let Some(request) = stream.recv().await {
        match request {
            Ok(request) => match request {
                Message::Text(request) => match deserialize::<S::Request>(&request) {
                    Ok(request) => {
                        let response = state.lock().await.handle(request);
                        match serialize(&response) {
                            Ok(response) => {
                                if let Err(e) = stream.send(Message::Text(response)).await {
                                    todo!()
                                }
                            }
                            Err(_) => todo!(),
                        }
                    }
                    Err(_) => todo!(),
                },
                Message::Binary(_) => todo!(),
                Message::Ping(_) => todo!(),
                Message::Pong(_) => todo!(),
                Message::Close(_) => todo!(),
            },
            Err(_) => todo!(),
        }
    }
    todo!()
}

// todo: replace this with generic serde schemes
#[cfg(feature = "serde_json")]
fn serialize<Response>(response: &Response) -> Result<String, serde_json::Error>
where
    Response: serde::Serialize,
{
    serde_json::to_string(response)
}

// todo: replace this with generic serde schemes
#[cfg(feature = "serde_json")]
fn deserialize<Request>(s: &str) -> Result<Request, serde_json::Error>
where
    Request: for<'a> serde::Deserialize<'a>,
{
    serde_json::from_str(s)
}
