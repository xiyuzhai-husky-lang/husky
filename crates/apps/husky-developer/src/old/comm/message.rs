mod notif;
mod request;
#[cfg(test)]
mod tests;

use common::Debug;
use serde::{Deserialize, Serialize};

use super::json::{ReadFromJSON, WriteToJSON};
use notif::Notification;
pub(super) use request::{Request, RequestId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum ClientMessage {
    Request(Request),
    Notification(Notification),
}

impl ReadFromJSON for ClientMessage {}
impl WriteToJSON for ClientMessage {}

impl From<Request> for ClientMessage {
    fn from(request: Request) -> ClientMessage {
        ClientMessage::Request(request)
    }
}

impl From<Notification> for ClientMessage {
    fn from(notification: Notification) -> ClientMessage {
        ClientMessage::Notification(notification)
    }
}
