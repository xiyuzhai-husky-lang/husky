use gemini_api::client::GeminiClient;

#[tokio::main]
async fn main() {
    let client = GeminiClient::new();
    let response = client
        .generate_content("Write a story about a magic backpack.")
        .await
        .unwrap();

    println!("Response: {}", response);
}
