use super::{ClientMessage, Notification, Request, RequestId};

#[test]
fn shutdown_with_explicit_null() {
    let text = "{\"jsonrpc\": \"2.0\",\"id\": 3,\"method\": \"shutdown\", \"params\": null }";
    let msg: ClientMessage = serde_json::from_str(&text).unwrap();

    assert!(
        matches!(msg, ClientMessage::Request(req) if req.id == 3.into() && req.method == "shutdown")
    );
}

#[test]
fn shutdown_with_no_params() {
    let text = "{\"jsonrpc\": \"2.0\",\"id\": 3,\"method\": \"shutdown\"}";
    let msg: ClientMessage = serde_json::from_str(&text).unwrap();

    assert!(
        matches!(msg, ClientMessage::Request(req) if req.id == 3.into() && req.method == "shutdown")
    );
}

#[test]
fn notification_with_explicit_null() {
    let text = "{\"jsonrpc\": \"2.0\",\"method\": \"exit\", \"params\": null }";
    let msg: ClientMessage = serde_json::from_str(&text).unwrap();

    assert!(matches!(msg, ClientMessage::Notification(not) if not.method == "exit"));
}

#[test]
fn notification_with_no_params() {
    let text = "{\"jsonrpc\": \"2.0\",\"method\": \"exit\"}";
    let msg: ClientMessage = serde_json::from_str(&text).unwrap();

    assert!(matches!(msg, ClientMessage::Notification(not) if not.method == "exit"));
}

#[test]
fn serialize_request_with_null_params() {
    let msg = ClientMessage::Request(Request {
        id: RequestId::from(3),
        method: "shutdown".into(),
        params: serde_json::Value::Null,
    });
    let serialized = serde_json::to_string(&msg).unwrap();

    assert_eq!("{\"id\":3,\"method\":\"shutdown\"}", serialized);
}

#[test]
fn serialize_notification_with_null_params() {
    let msg = ClientMessage::Notification(Notification {
        method: "exit".into(),
        params: serde_json::Value::Null,
    });
    let serialized = serde_json::to_string(&msg).unwrap();

    assert_eq!("{\"method\":\"exit\"}", serialized);
}
