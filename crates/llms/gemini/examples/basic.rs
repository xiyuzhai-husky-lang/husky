use std::path::PathBuf;
use tempfile::TempDir;

use eterned::db::EternerDb;
use gemini::client::GeminiClient;

fn main() {
    let db = EternerDb::default();
    let temp_dir = TempDir::new().unwrap();
    let cache_path = temp_dir.path().join("gemini_responses.json");
    let client = GeminiClient::new(&db, cache_path).unwrap();
    let response = client
        .generate_text("Write a story about a magic backpack.")
        .unwrap();

    println!("Response: {}", response);
}
