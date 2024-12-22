use std::path::PathBuf;

use eterned::db::EternerDb;
use gemini::client::GeminiClient;

fn main() {
    let db = EternerDb::default();
    let client = GeminiClient::new(&db, PathBuf::from("gemini_cache")).unwrap();
    let response = client
        .generate_text("Write a story about a magic backpack.")
        .unwrap();

    println!("Response: {}", response);
}
