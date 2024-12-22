use gemini_api::client::GeminiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GeminiClient::new();
    for i in 0..20 {
        let response = client
            .generate_content("Write a story about a magic backpack.")
            .await?;
        let response_str = response.to_string();
        if response_str.len() > 50 {
            println!("{i}th response: {:.50}...", response_str);
        } else {
            println!("{i}th response: {}", response_str);
        }
    }

    Ok(())
}
