use husky_print_utils::p;

const SERVER_ADDRESS: &str = "ws://localhost:51718/ws";

#[tokio::main]
async fn main() {
    match tokio_tungstenite::connect_async(SERVER_ADDRESS).await {
        Ok((stream, response)) => {
            todo!()
        }
        Err(e) => {
            p!(e);
            todo!()
        }
    }
}
