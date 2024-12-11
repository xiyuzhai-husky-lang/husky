use callm::pipelines::PipelineText;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    model_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    // Parse command line arguments
    let cli = Cli::parse();

    println!("model_path: {}", cli.model_path);

    // Build pipeline
    let mut pipeline = PipelineText::builder()
        .with_location(&cli.model_path)
        .build()?;

    // Run inference
    let text_completion = pipeline.run("Tell me a joke about Rust borrow checker")?;
    println!("{text_completion}");

    Ok(())
}
