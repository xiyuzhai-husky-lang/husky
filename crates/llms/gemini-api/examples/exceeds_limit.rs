use gemini_api::client::GeminiClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let client = GeminiClient::new(api_key);
    for i in 0..10 {
        let response = client
            .generate_content("Write a story about a magic backpack.")
            .await?;
        println!("{i}th response: {}", response);
    }

    Ok(())
}
