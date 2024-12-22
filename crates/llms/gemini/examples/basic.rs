use std::path::PathBuf;
use tempfile::TempDir;

use eterned::db::EternerDb;
use gemini::{client::GeminiClient, model::GeminiModel};

fn main() {
    let db = EternerDb::default();
    let cache_dir = TempDir::new().unwrap();
    let model = GeminiModel::Gemini1_5Flash;
    let client = GeminiClient::new(&db, cache_dir.path()).unwrap();
    let response = client
        .generate_text(model, "Write a story about a magic backpack.")
        .unwrap();

    println!("Response: {}", response);
}
