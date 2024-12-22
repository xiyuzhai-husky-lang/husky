use eterned::db::EternerDb;
use gemini::client::GeminiClient;
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
    let temp_dir = TempDir::new().map_err(|e| {
        error!("Failed to create temp directory: {}", e);
        e
    })?;

    let cache_path = temp_dir.path().join("gemini_responses.json");
    let client = GeminiClient::new(&db, cache_path).map_err(|e| {
        error!("Failed to create Gemini client: {}", e);
        e
    })?;

    for i in 0..20 {
        info!("Generating response for i={}", i);
        let response = match client.generate_text(&format!(
            "What is {i} + {i} equal to? Just give the number."
        )) {
            Ok(resp) => resp,
            Err(e) => {
                error!("Failed to generate text for i={}: {}", i, e);
                continue;
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
