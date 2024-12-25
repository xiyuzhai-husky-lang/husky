use alien_seed::AlienSeed;
use clap::Parser;
use eterned::db::EternerDb;
use glob::glob;
use std::{path::PathBuf, sync::Arc};
use visored_pipeline::{error::VdPipelineResult, runner::VdPipelineRunner};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "visored-config.yaml")]
    config: PathBuf,
    src_files: Vec<String>,
}

impl Cli {
    fn expanded_src_file_paths(&self) -> Vec<PathBuf> {
        self.src_files
            .iter()
            .flat_map(|pattern| {
                glob(pattern)
                    .expect("Failed to read glob pattern")
                    .filter_map(Result::ok)
            })
            .collect()
    }
}

fn main() {
    let db = &EternerDb::default();
    let cli = Cli::parse();
    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let src_file_paths = cli.expanded_src_file_paths();
    match run(db, tokio_runtime, cli.config, src_file_paths) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run(
    db: &EternerDb,
    tokio_runtime: Arc<tokio::runtime::Runtime>,
    config_path: PathBuf,
    src_file_paths: Vec<PathBuf>,
) -> VdPipelineResult<()> {
    let mut runner = VdPipelineRunner::new(db, tokio_runtime, config_path, src_file_paths)?;
    runner.run_all_single_threaded(AlienSeed::new(0))?;
    Ok(())
}
