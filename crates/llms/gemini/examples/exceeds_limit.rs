use eterned::db::EternerDb;
use gemini::{client::GeminiClient, model::GeminiModel};
use std::path::PathBuf;
use tempfile::TempDir;
use tracing::{error, info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    let db = EternerDb::default();
    let cache_dir = &TempDir::new().map_err(|e| {
        error!("Failed to create temp directory: {}", e);
        e
    })?;

    let client = GeminiClient::new(&db, cache_dir.path()).map_err(|e| {
        error!("Failed to create Gemini client: {}", e);
        e
    })?;

    let model = GeminiModel::Gemini1_5Flash;

    for i in 0..20 {
        info!("Generating response for i={}", i);
        let response = match client.generate_text(
            model,
            &format!("What is {i} + {i} equal to? Just give the number."),
        ) {
            Ok(resp) => resp,
            Err(e) => {
                panic!("Failed to generate text for i={}: {}", i, e);
            }
        };

        let response_str = response.to_string();
        if response_str.len() > 50 {
            warn!("Response exceeded 50 characters");
            println!("{i}th response: {:.50}...", response_str);
        } else {
            info!("Got response of {} characters", response_str.len());
            println!("{i}th response: {}", response_str);
        }
    }

    Ok(())
}
