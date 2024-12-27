use alien_seed::{attach::with_seed, AlienSeed};
use eterned::db::EternerDb;
use gemini::{client::GeminiClient, model::GeminiModel};
use std::{path::PathBuf, sync::Arc};
use tempfile::TempDir;

fn main() {
    let db = EternerDb::default();
    let seed = AlienSeed::new(0);
    let cache_dir = TempDir::new().unwrap();
    let model = GeminiModel::Gemini1_5Flash;
    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let client = GeminiClient::new(&db, tokio_runtime, cache_dir.path()).unwrap();
    let response = with_seed(seed, || {
        client
            .generate_text(model, "Write a story about a magic backpack.")
            .unwrap()
    });

    println!("Response: {}", response);
}
