use eterned::db::EternerDb;
use gemini::client::GeminiClient;
use std::path::PathBuf;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = EternerDb::default();
    let temp_dir = TempDir::new().unwrap();
    let cache_path = temp_dir.path().join("gemini_responses.json");
    let client = GeminiClient::new(&db, cache_path).unwrap();
    for i in 0..20 {
        let response = client
            .generate_text("Write a story about a magic backpack.")
            .unwrap();
        let response_str = response.to_string();
        if response_str.len() > 50 {
            println!("{i}th response: {:.50}...", response_str);
        } else {
            println!("{i}th response: {}", response_str);
        }
    }

    Ok(())
}
